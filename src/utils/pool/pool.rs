use std::sync::Arc;
use std::sync::Mutex;
use std::vec::Vec;

pub struct ObjectPool<T> {
    pool: Arc<Mutex<Vec<T>>>,
    create_fn: Box<dyn Fn() -> T + Send + Sync>,
    max_size: usize,
}

impl<T> ObjectPool<T> {
    pub fn new(create_fn: Box<dyn Fn() -> T + Send + Sync>, max_size: usize) -> Self {
        Self {
            pool: Arc::new(Mutex::new(Vec::with_capacity(max_size))),
            create_fn,
            max_size,
        }
    }

    pub fn get(&self) -> T {
        let mut pool = self.pool.lock().unwrap();
        if let Some(obj) = pool.pop() {
            obj
        } else {
            (self.create_fn)()
        }
    }

    pub fn put(&self, obj: T) {
        let mut pool = self.pool.lock().unwrap();
        if pool.len() < self.max_size {
            pool.push(obj);
        }
    }

    pub fn clear(&self) {
        let mut pool = self.pool.lock().unwrap();
        pool.clear();
    }

    pub fn size(&self) -> usize {
        self.pool.lock().unwrap().len()
    }

    pub fn max_size(&self) -> usize {
        self.max_size
    }

    pub fn is_empty(&self) -> bool {
        self.pool.lock().unwrap().is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.pool.lock().unwrap().len() >= self.max_size
    }
}

impl<T: Default> ObjectPool<T> {
    pub fn new_default(max_size: usize) -> Self {
        Self::new(Box::new(T::default), max_size)
    }
}
