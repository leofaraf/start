use std::{cell::RefCell, rc::Rc};

use super::storage::start_storage::StartStorage;

/// Write operation, contains information:
/// 
/// Offset to certain place in physical db
/// 
/// Old-data for rollback
/// 
/// New data to write for commit
pub struct WriteOp {
    offset: usize,
    new_data: Vec<u8>,
    old_data: Vec<u8>,
}

/// RecoveryUnit is core concept of database
/// 
/// It provides utils for `Atomic transactions`
/// 
/// Stores changes that user written inside
///
/// Commit it to storage on commit operation,
/// 
/// or rollback it
pub struct RecoveryUnit {
    storage: Rc<RefCell<StartStorage>>,
    pending_ops: Vec<WriteOp>,
    committed: bool
}

impl RecoveryUnit {
    pub fn new(storage: Rc<RefCell<StartStorage>>) -> Self {
        Self {
            storage,
            pending_ops: vec![],
            committed: false,
        }
    }

    /// Operation to add pending operation, that might be written on storage on commit
    pub fn write(&mut self, offset: usize, data: &[u8]) {
        let old = self.storage.borrow()[offset..offset+data.len()].to_vec();

        self.pending_ops.push(WriteOp {
            offset,
            new_data: data.to_vec(),
            old_data: old,
        });
    }

    /// Apply all actual pending operations
    pub fn commit(&mut self) {
        let mut ss = self.storage.borrow_mut();
        for op in self.pending_ops.iter() {
            println!("Commiting op");
            println!("{}: '{:?}' to '{:?}'", op.offset, op.old_data, op.new_data);
            ss[op.offset..op.offset+op.new_data.len()].copy_from_slice(&op.new_data);
        }
        self.committed = true;
    }

    /// Clear pending operations
    pub fn rollback(&mut self) {
        self.pending_ops.clear();
    }

    /// Gives actual content of database with apply of recovery unit context
    pub fn effective_view(&self, offset: usize, len: usize) -> Vec<u8> {
        // Start with the base data from the storage
        let mut result = self.storage.borrow()[offset..offset + len].to_vec();

        // Apply all pending writes that affect this range
        for op in &self.pending_ops {
            let op_start = op.offset;
            let op_end = op.offset + op.new_data.len();
            let view_start = offset;
            let view_end = offset + len;

            // Find overlap
            if op_end > view_start && op_start < view_end {
                let overlap_start = op_start.max(view_start);
                let overlap_end = op_end.min(view_end);
                let result_start = overlap_start - view_start;
                let op_data_start = overlap_start - op_start;

                let count = overlap_end - overlap_start;
                result[result_start..result_start + count]
                    .copy_from_slice(&op.new_data[op_data_start..op_data_start + count]);
            }
        }

        result
    }

    pub fn is_committed(&self) -> bool {
        self.committed
    }
}

/// Like RAII in Mongo DB
impl Drop for RecoveryUnit {
    fn drop(&mut self) {
        if !self.committed {
            self.rollback();
        }
    }
}

#[test]
fn test_atomic_commit_and_rollback() {
    use std::rc::Rc;
    use std::cell::RefCell;

    // Set up initial storage
    let storage = Rc::new(RefCell::new(StartStorage::in_memory()));
    storage.borrow_mut().resize(16).unwrap();
    {
        let mut s = storage.borrow_mut();
        s[0..4].copy_from_slice(&[1, 2, 3, 4]);
        s[4..8].copy_from_slice(&[5, 6, 7, 8]);
    }

    // Simulate commit
    {
        let mut ru = RecoveryUnit::new(storage.clone());
        ru.write(0, &[10, 11, 12, 13]);
        ru.write(4, &[20, 21, 22, 23]);
        ru.commit();
        assert!(ru.is_committed());
    }

    {
        let s = storage.borrow();
        assert_eq!(&s[0..4], &[10, 11, 12, 13]);
        assert_eq!(&s[4..8], &[20, 21, 22, 23]);
    }

    // Simulate rollback
    {
        let mut ru = RecoveryUnit::new(storage.clone());
        ru.write(0, &[100, 101, 102, 103]);
        ru.write(4, &[200, 201, 202, 203]);
        assert!(!ru.is_committed()); // hasn't been committed yet
        // ru drops without commit => triggers rollback
    }

    {
        let s = storage.borrow();
        assert_eq!(&s[0..4], &[10, 11, 12, 13]);
        assert_eq!(&s[4..8], &[20, 21, 22, 23]);
    }
}
