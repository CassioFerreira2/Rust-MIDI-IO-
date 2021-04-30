
use std::{mem, ops::{Deref, DerefMut}};

pub mod traits;
use traits::*;

pub struct ModernVec<T> {
    _lock: Option<usize>,
    _vec: Vec<T>,
}

impl<T> ModernVec<T> {
    pub fn new() -> ModernVec<T> {
        ModernVec {_vec: Vec::new(), _lock: None}
    }
}

impl<T> ReplacebleIndex<T> for ModernVec<T> {
    /// Will replace at the specified index
    fn replace(&mut self, index: usize, mut src: T) {
        mem::swap(&mut self._vec[index], &mut src);
    }
}

impl<T> Lockable<T> for ModernVec<T> {

    /// Lock will block the vector at the specified length
    fn lock(&mut self, len: usize) {
        self._lock = Some(len);
    }

    /// Safe push will block any push that is greater than the lock
    fn safe_push(&mut self, src: T) {
        if let Some(val) = self._lock {
            if self._vec.len() < val {
                self._vec.push(src)
            } else {
                panic!(
                    "You cannot push more, the vector is locked at {}", val
                )
            }
        } else {
            panic!("You are using safe_push without marked the lock!")
        }
    }
}

impl<T> Deref for ModernVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self._vec
    }
}

impl<T> DerefMut for ModernVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._vec
    }
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let mut m_vec = ModernVec::new();
        m_vec.push(1);
        m_vec.push(3);
        m_vec.push(5);

        m_vec.replace(1, 2);
        assert_eq!(m_vec[1], 2);

        m_vec.replace(1, 3);
        assert_eq!(m_vec[1], 3);

        assert_eq!(m_vec[2], 5);
        m_vec.replace(2, 0);
        assert_eq!(m_vec[2], 0);
    }

    #[test]
    fn test_index2() {
        let mut m_vec: ModernVec<char> = ModernVec::new();

        m_vec.push('a');
        m_vec.push('2');
        m_vec.push('5');

        m_vec.replace(0, 'b');
        assert_eq!(m_vec[0], 'b');

        m_vec.replace(0, '2');
        assert_eq!(m_vec[0], '2');
    }

    #[test]
    #[should_panic]
    fn test_index3() {
        let mut m_vec: ModernVec<char> = ModernVec::new();

        m_vec.push('a');
        m_vec.push('2');
        m_vec.push('5');

        m_vec.replace(4, 'b');
    }

    #[test]
    fn test_lock1() {
        let mut m_vec = ModernVec::new();

        m_vec.lock(5);

        m_vec.safe_push(2);
        m_vec.safe_push(2);
        m_vec.safe_push(2);
        m_vec.safe_push(2);
        m_vec.safe_push(2);
    }

    #[test]
    #[should_panic]
    fn test_lock2() {
        let mut m_vec = ModernVec::new();

        m_vec.lock(5);

        m_vec.safe_push(2);
        m_vec.safe_push(2);
        m_vec.safe_push(2);
        m_vec.safe_push(2);
        m_vec.safe_push(2);
        m_vec.safe_push(2);
    }
}


