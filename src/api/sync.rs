use std::sync::{Arc, Mutex};
use crate::api::err;
use once_cell::sync::OnceCell;

#[derive(Clone)]

pub struct NeedPublish(Arc<Mutex<bool>>);
pub type ConnPool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub struct GlobalConnPool(pub ConnPool);

impl GlobalConnPool{
    pub fn global() -> &'static GlobalConnPool {
        CONN_POOL.get().expect("Global Conn Pool is not initialized")
    }

    fn init() -> Result<GlobalConnPool, err::Error> {
        unimplemented!();
    }
}

pub static CONN_POOL: OnceCell<GlobalConnPool> = OnceCell::new();


impl NeedPublish {
    pub fn set(&self, state: bool) {
        // Poison of this mutex is fatal
        *self.0.lock().unwrap() = state;
    }
    pub fn get(&self) -> bool {
        *self.0.lock().unwrap()
    }

    fn new(state: bool) -> Self {
        NeedPublish(Arc::new(Mutex::new(state)))
    }
}
