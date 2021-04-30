
use std::{mem, ops::{Deref, DerefMut}};

pub mod traits;
use traits::*;


pub struct AsciiVec {
    _vec: ModernVec<char>
}

impl AsciiVec {
    pub fn new(len: usize) -> AsciiVec {
        let mut _vec = ModernVec::new();
        _vec.lock(len);
        
        let mut av = AsciiVec {_vec};
        av.fill(' ');

        av
    }

    /// The difference from this fill from the other normal vector fill is that
    /// this will not erase the \n
    pub fn fill(&mut self, ch: char) {
        let lock_len = self._vec.get_lock_len();

        for i in 0..lock_len {
            if let None = self._vec.get(i) {
                self._vec.safe_push(ch)
            } else {
                self._vec.replace(i, ch)
            }
        }

        self.set_last_n();
    }

    /// This get will be more safe, because it will not catch \n characters or out of bounds index
    pub fn get(&self, ind: usize) -> char {
        let ch = self._vec[ind];

        if ch == '\n' {panic!("Getting index {}, but character is \\n ", ind)}
        ch
    }

    /// Will replace only if the char is not \n
    pub fn replace(&mut self, index: usize, src: char) {
        let ch = self._vec[index];
        if ch != '\n' {
            self._vec.replace(index, src);
        } else {
            panic!("Trying to replace char that is break line, index: {}, char: {}", index, src)
        }
    }

    pub fn get_str(&self) -> String {
        let s: String = self._vec.iter().collect();
        s
    }

    fn set_last_n(&mut self) {
        self._vec.replace(self._vec.get_lock_len()-1, '\n');
    }


}




// ---------------------------------------- // ------------------------------------------- //







pub struct ModernVec<T> {
    _lock: Option<usize>,
    _vec: Vec<T>,
}

impl<T> ModernVec<T> {
    pub fn new() -> ModernVec<T> {
        ModernVec {_vec: Vec::new(), _lock: None}
    }

    /// Safe push will block any push that is greater than the lock
    pub fn safe_push(&mut self, src: T) {
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

    fn is_locked(&self) -> bool {
        if let Some(v) = self._lock {
            true
        } else {
            false
        }
    }

    fn get_lock_len(&self) -> usize {
        self._lock.expect("Trying to get lock len when no lock is defined")
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

    // ---------------------- ASCII VEC -------------------------- //
    #[test]
    fn test_fill() {
        let mut a_vec = AsciiVec::new(5);
        a_vec.fill('b');

        assert_eq!(a_vec.get(0), 'b');
        assert_eq!(a_vec.get(1), 'b');
        assert_eq!(a_vec.get(2), 'b');
        assert_eq!(a_vec.get(3), 'b');
        
        a_vec.fill(' ');
        assert_eq!(a_vec.get(0), ' ');
        assert_eq!(a_vec.get(1), ' ');
        assert_eq!(a_vec.get(2), ' ');
        assert_eq!(a_vec.get(3), ' ');
    }

    #[test]
    #[should_panic]
    fn test_fill2() {
        let mut a_vec = AsciiVec::new(5);
        a_vec.fill('b');

        assert_eq!(a_vec.get(0), 'b');
        assert_eq!(a_vec.get(1), 'b');
        assert_eq!(a_vec.get(2), 'b');
        assert_eq!(a_vec.get(3), 'b');
        assert_eq!(a_vec.get(4), 'b');  // error because lock is 5
    }   

    #[test]
    fn test_ascii_replace() {
        let mut a_vec = AsciiVec::new(5);
        a_vec.fill('b');

        a_vec.replace(2, 'c');

        assert_eq!(a_vec.get(0), 'b');
        assert_eq!(a_vec.get(1), 'b');
        assert_eq!(a_vec.get(2), 'c');
        assert_eq!(a_vec.get(3), 'b');
    }

    #[test]
    #[should_panic]
    fn test_ascii_replace2() {
        let mut a_vec = AsciiVec::new(5);
        a_vec.fill('b');

        a_vec.replace(5, 'c');  // index five == \n
    }

    #[test]
    #[should_panic]
    fn test_ascii_replace3() {
        let mut a_vec = AsciiVec::new(5);
        a_vec.fill('b');

        a_vec.replace(65, 'c');  // no exists
    }

    #[test]
    fn test_get_str() {
        let mut a_vec = AsciiVec::new(12);
        a_vec.fill('b');

        a_vec.replace(2, 'c');
        a_vec.replace(0, 'o');

        let st = a_vec.get_str();

        assert_eq!("obcbbbbbbbb\n", st);
    }

    // ---------------------- MODERN VEC ------------------------- //
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


