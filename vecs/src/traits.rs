pub trait ReplacebleIndex<T> {
    fn replace(&mut self, index: usize, src: T);
}

pub trait Lockable<T> {
    fn lock(&mut self, len: usize);
    fn safe_push(&mut self, value: T);
}


