use crate::{CustomElement, Identity};

pub struct Registry<T> {
    register : Vec<Option<T>>
}

impl<T> Registry<T> {
    pub fn object(&mut self, object: Identity) -> Option<&mut T> {
        self
            .register
            .get_mut(object)
            .and_then(|object| object.as_mut())
    }

    pub fn register_object(&mut self, object: T) -> Identity {
        self.register.push(Some(object));
        self.register.len() - 1
    }

    pub fn remove(&mut self, object: Identity) -> Option<T> {
        self
            .register
            .get_mut(object)
            .and_then(Option::take)
    }
}

pub static mut REGISTRY: Registry<Box<dyn CustomElement>> = Registry {
    register : Vec::new()
};
