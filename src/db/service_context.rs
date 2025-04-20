use std::{cell::{Ref, RefCell}, path::PathBuf, rc::Rc, sync::Arc};

use start_storage::StartStorage;

use crate::HandleResult;

use super::{catalog::Catalog, header::{get_header, Header}, operation_context::OperationContext};

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
    let storage = Rc::new(RefCell::new(raw_storage));

    // Init operation context
    let header = get_header(storage.clone());
    let catalog = Rc::new(RefCell::new(Catalog::new(storage.clone())));

    let service_context = ServiceContext {
        storage,
        catalog,
        header
    };

    service_context
}

pub fn embedded(path: PathBuf) -> HandleResult<ServiceContext> {
    let mut raw_storage = StartStorage::embedded(path)?;
    let storage = Rc::new(RefCell::new(raw_storage));
    let header = get_header(storage.clone());
    let catalog = Rc::new(RefCell::new(Catalog::new(storage.clone())));

    let service_context = ServiceContext {
        storage,
        catalog,
        header
    };

    Ok(service_context)
}