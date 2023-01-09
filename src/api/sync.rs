use std::sync::{Arc, Mutex};

pub struct NeedPublish(Arc<Mutex<bool>>);

impl NeedPublish{
    fn set(&self, state: bool){
        // Poison of this mutex is fatal
        *self.0.lock().unwrap() = state;
    }
    fn get(&self) -> bool {
        *self.0.lock().unwrap()
    }
}