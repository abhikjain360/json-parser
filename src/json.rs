use std::any::Any;
use std::collections::HashMap;

pub struct Json(HashMap<String, Box<dyn Any>>);

impl Json {
    pub fn new() -> Self {
        Json(HashMap::new())
    }

    pub fn insert(&mut self, key: String, value: Box<dyn Any>) {
        self.0.insert(key, value);
    }

    pub fn get<T: 'static>(&self, key: &str) -> Option<&T> {
        // self.0.get(key).map(|boxed| boxed.downcast_ref::<T>().unwrap())
        match self.0.get(key) {
            Some(boxed) => Some(boxed.downcast_ref::<T>()?),
            None => None
        }
    }

    pub fn get_mut<T: 'static>(&mut self, key: &str) -> Option<&mut T> {
        match self.0.get_mut(key) {
            Some(boxed) => Some(boxed.downcast_mut::<T>()?),
            None => None
        }
    }
}

impl From<HashMap<String, Box<dyn Any>>> for Json {
    fn from(map: HashMap<String, Box<dyn Any>>) -> Self {
        Json(map)
    }
}
