use std::{cell::RefCell, path::PathBuf, rc::Rc};

use crate::HandleResult;

use super::{catalog::{session::SessionCatalog, Catalog}, header::get_header, operation_context::OperationContext, storage::start_storage::StartStorage};

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

pub fn in_memory() -> Rc<ServiceContext> {
    let raw_storage = StartStorage::in_memory();
    let storage = Rc::new(RefCell::new(raw_storage));
    let catalog = Rc::new(RefCell::new(Catalog::new(storage.clone())));

    let service_context = Rc::new(ServiceContext {
        storage,
        catalog,
    });

    // Init operation context

    let init_session = SessionCatalog::acquire(service_context.clone());
    let mut init_op_ctx = OperationContext::new(&init_session);
    let _header = get_header(&mut init_op_ctx);
    init_op_ctx.rc_unit().borrow_mut().commit();

    service_context
}

pub fn embedded(path: PathBuf) -> HandleResult<Rc<ServiceContext>> {
    let raw_storage = StartStorage::embedded(path)?;
    let storage = Rc::new(RefCell::new(raw_storage));
    let catalog = Rc::new(RefCell::new(Catalog::new(storage.clone())));

    let service_context = Rc::new(ServiceContext {
        storage,
        catalog,
    });

    // Init operation context
    
    let init_session = SessionCatalog::acquire(service_context.clone());
    let mut init_op_ctx = OperationContext::new(&init_session);
    let _header = get_header(&mut init_op_ctx);
    init_op_ctx.rc_unit().borrow_mut().commit();

    Ok(service_context)
}