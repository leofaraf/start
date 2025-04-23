use std::{cell::RefCell, path::PathBuf, rc::Rc};

use crate::HandleResult;

use super::{catalog::Catalog, header::get_header, operation_context::OperationContext, storage::start_storage::StartStorage};

pub struct ServiceContext {
    storage: Rc<RefCell<StartStorage>>,
    catalog: Rc<RefCell<Catalog>>,
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
    let raw_storage = StartStorage::in_memory();
    let storage = Rc::new(RefCell::new(raw_storage));
    let catalog = Rc::new(RefCell::new(Catalog::new(storage.clone())));

    let service_context = ServiceContext {
        storage,
        catalog,
    };

    // Init operation context

    let init_op_ctx = OperationContext::new(&service_context);
    let _header = get_header(init_op_ctx);

    service_context
}

pub fn embedded(path: PathBuf) -> HandleResult<ServiceContext> {
    let raw_storage = StartStorage::embedded(path)?;
    let storage = Rc::new(RefCell::new(raw_storage));
    let catalog = Rc::new(RefCell::new(Catalog::new(storage.clone())));

    let service_context = ServiceContext {
        storage,
        catalog,
    };

    // Init operation context

    let init_op_ctx = OperationContext::new(&service_context);
    let _header = get_header(init_op_ctx);

    Ok(service_context)
}