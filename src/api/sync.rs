use once_cell::sync::OnceCell;
use std::sync::Mutex;

/// Global Database Connection Pool
pub type ConnPool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
#[derive(Debug)]
pub struct GlobalConnPool(pub ConnPool);

impl GlobalConnPool {
    pub fn global() -> &'static GlobalConnPool {
        CONN_POOL
            .get()
            .expect("Global Conn Pool is not initialized")
    }
}

pub static CONN_POOL: OnceCell<GlobalConnPool> = OnceCell::new();

/// A global var indicating the necessity to deploy
#[derive(Debug)]
pub struct NeedPublish(Mutex<bool>);

pub static NEED_PUBLISH: OnceCell<NeedPublish> = OnceCell::new();

impl NeedPublish {
    pub fn set(&self, state: bool) {
        // Poison of this mutex is fatal
        *self.0.lock().unwrap() = state;
    }
    pub fn get(&self) -> bool {
        *self.0.lock().unwrap()
    }

    pub fn new(state: bool) -> Self {
        NeedPublish(Mutex::new(state))
    }

    pub fn global() -> &'static NeedPublish {
        NEED_PUBLISH.get().expect("Error Getting Need Publish Var")
    }
}
