use std::{cell::{RefCell, RefMut}, rc::Rc};

use start_storage::StartStorage;

pub struct WriteOp {
    pub offset: usize,
    pub new_data: Vec<u8>,
    pub old_data: Vec<u8>,
}

pub struct RecoveryUnit {
    pub storage: Rc<RefCell<StartStorage>>,
    pub pending_ops: Vec<WriteOp>,
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

    pub fn write(&mut self, offset: usize, data: &[u8]) {
        // Capture the current data for rollback
        let old = self.storage.borrow()[offset..offset+data.len()].to_vec();

        self.pending_ops.push(WriteOp {
            offset,
            new_data: data.to_vec(),
            old_data: old,
        });
    }

    pub fn commit(&mut self) {
        let mut ss = self.storage.borrow_mut();
        for op in self.pending_ops.iter().rev() {
            println!("Commiting op");
            println!("{}: '{:?}' to '{:?}'", op.offset, op.old_data, op.new_data);
            ss[op.offset..op.offset+op.new_data.len()].copy_from_slice(&op.new_data);
        }
        self.committed = true;
    }

    pub fn rollback(&mut self) {
        let mut ss = self.storage.borrow_mut();
        for op in self.pending_ops.iter().rev() {
            println!("Rolling back op");
            ss[op.offset..op.offset+op.old_data.len()].copy_from_slice(&op.old_data);
        }
        self.pending_ops.clear();
    }
}

impl Drop for RecoveryUnit {
    fn drop(&mut self) {
        if !self.committed {
            self.rollback();
        }
    }
}