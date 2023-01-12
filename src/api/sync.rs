use std::sync::{Arc, Mutex};

#[derive(Clone)]

pub struct NeedPublish(Arc<Mutex<bool>>);
pub type ConnPool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

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
