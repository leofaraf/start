use std::{cell::{Ref, RefCell}, path::PathBuf, rc::Rc, sync::Arc};

use start_storage::StartStorage;

use crate::HandleResult;

use super::{catalog::Catalog, header::{get_header, Header}};

pub struct ServiceContext {
    storage: Rc<RefCell<StartStorage>>,
    catalog: Rc<RefCell<Catalog>>,
    header: Header
}

impl ServiceContext {
    pub fn storage(&self) -> Rc<RefCell<StartStorage>> {
        Rc::clone(&self.storage)
    }

    pub fn catalog(&self) -> Rc<RefCell<Catalog>> {
        Rc::clone(&self.catalog)
    }
}

pub fn in_memory() -> ServiceContext {
    let mut raw_storage = StartStorage::in_memory();
    let header = get_header(&mut raw_storage);
    let storage = Rc::new(RefCell::new(raw_storage));
    let catalog = Rc::new(RefCell::new(Catalog::new())); // or Catalog::load(&storage)

    let service_context = ServiceContext {
        storage,
        catalog,
        header
    };

    service_context
}

pub fn embedded(path: PathBuf) -> HandleResult<ServiceContext> {
    let mut raw_storage = StartStorage::embedded(path)?;
    let header = get_header(&mut raw_storage);
    let storage = Rc::new(RefCell::new(raw_storage));
    let catalog = Rc::new(RefCell::new(Catalog::new())); // or Catalog::load(&storage)

    let service_context = ServiceContext {
        storage,
        catalog,
        header
    };

    Ok(service_context)
}