pub trait ReplacebleIndex<T> {
    fn replace(&mut self, index: usize, src: T);
}

pub trait Lockable<T> {
    fn lock(&mut self, len: usize);
    fn get_lock_len(&self) -> usize;
    fn is_locked(&self) -> bool;
}


