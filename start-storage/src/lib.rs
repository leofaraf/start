use std::{error::Error, fs::{File, OpenOptions}, ops::{Index, IndexMut}, path::PathBuf};

use memmap2::MmapMut;

type HandleResult<T> = Result<T, Box<dyn Error>>;

pub enum StartStorage {     
    InMemory(Vec<u8>),
    Mapped {
        path: PathBuf,
        mmap: MmapMut
    }
}

impl StartStorage {
    pub fn len(&self) -> usize {
        match self {
            StartStorage::InMemory(vec) => vec.len(),
            StartStorage::Mapped { mmap, .. } => mmap.len(),
        }
    }

    pub fn resize(&mut self, new_len: usize) -> HandleResult<()> {
        match self {
            StartStorage::InMemory(vec) => {
                vec.resize(new_len, 0);
            },
            StartStorage::Mapped { mmap, path } => {
                let file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(&path)?;
                file.set_len(new_len as u64)?;

                let new_mmap = unsafe { MmapMut::map_mut(&file) }?;
                *mmap = new_mmap;
            },
        }
        Ok(())
    }

    pub fn in_memory() -> Self {
        Self::InMemory(vec![])
    }

    pub fn embedded(path: PathBuf) -> HandleResult<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
        let mmap = unsafe { MmapMut::map_mut(&file) }?;
        Ok(Self::Mapped { path, mmap })
    }
}

impl Index<std::ops::Range<usize>> for StartStorage {
    type Output = [u8];

    fn index(&self, range: std::ops::Range<usize>) -> &Self::Output {
        match &self {
            StartStorage::InMemory(vec) => &vec[range],
            StartStorage::Mapped{ path: _, mmap } => &mmap[range],
        }
    }
}

impl IndexMut<std::ops::Range<usize>> for StartStorage {
    fn index_mut(&mut self, range: std::ops::Range<usize>) -> &mut Self::Output {
        match self {
            StartStorage::InMemory(vec) => &mut vec[range],
            StartStorage::Mapped { mmap, .. } => &mut mmap[range],
        }
    }
}