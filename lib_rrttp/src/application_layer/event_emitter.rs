use std::collections::HashMap;

pub struct EventEmitter<T> {
    subscribers: HashMap<&'static str, Box<dyn Fn(&T) + Sync + Send>>,
}

impl<T> EventEmitter<T> {
    pub fn new() -> Self {
        Self {
            subscribers: HashMap::new()
        }
    }

    pub fn subscribe(&mut self, key: &'static str, subscriber: Box<dyn Fn(&T) + Sync + Send>) {
        self.subscribers.insert(key, subscriber);
    }

    pub fn unsubscribe(&mut self, key: &str) {
        self.subscribers.remove(key);
    }

    pub fn notify(&self, info: &T) {
        for subscriber in self.subscribers.values() {
            subscriber(info);
        }
    }
}