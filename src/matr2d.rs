/// You probably will recognize that the width is lower than what is printed.
/// Well... this isn't a bug, what is happening is that '\n' is a hidden special char,
/// but it's a char so the width must include him 

use vecs::AsciiVec;
use vecs::ModernVec;
use vecs::traits::*;

pub struct Matr2D {
    _row_vec: ModernVec<AsciiVec>,
}

impl Matr2D {
    fn new(width: usize, height: usize, fill_with: Option<char>) -> Matr2D {
        let mut _row_vec = ModernVec::new();
        _row_vec.lock(height);

        // ascii vec already have lock
        let mut a_vec = AsciiVec::new(width);

        // will fill ascii vec with specified char
        if let Some(c) = fill_with {
            a_vec.fill(c);
        } else {
            a_vec.fill('x');
        
        }
        // all rows in the vector will have the same ascii vec 
        for _ in 0.._row_vec.get_lock_len() {
            _row_vec.safe_push(a_vec.clone())
        }
        Matr2D {_row_vec}
    }
    
    // We dont need to do so much error checks because the vecs already have a lot
    fn draw_point(&mut self, x: usize, y: usize, ch: char) {
        let row = &mut self._row_vec[y];
        row.replace(x, ch);
    }

    fn fill(&mut self, ch: char) {
        for i in 0..self._row_vec.get_lock_len() {
            self._row_vec[i].fill(ch);
        }
    }

    fn get_str(&mut self) -> String {
        let mut rows: Vec<String> = Vec::new();

        for i in 0..self._row_vec.get_lock_len() {
            rows.push(self._row_vec[i].get_str());      // read error E0716 to see why String over &str in this case
        }

        let st: String = rows.concat();
        st
    }
}

// ---------------- TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut mat = Matr2D::new(4, 4, Some('a'));
        let result = mat.get_str();
        let expect = String::from("aaa\naaa\naaa\naaa\n");

        assert_eq!(result, expect);
    }

    #[test]
    fn test_new2() {
        let mut mat = Matr2D::new(4, 4, None);
        let result = mat.get_str();
        let expect = String::from("xxx\nxxx\nxxx\nxxx\n");

        assert_eq!(result, expect);
    }

    #[test]
    fn test_new3() {
        let mut mat = Matr2D::new(4, 7, None);
        let result = mat.get_str();
        let expect = String::from("xxx\nxxx\nxxx\nxxx\nxxx\nxxx\nxxx\n");

        assert_eq!(result, expect);
    }

    #[test]
    fn test_new4() {
        let mut mat = Matr2D::new(7, 4, None);
        let result = mat.get_str();
        let expect = String::from("xxxxxx\nxxxxxx\nxxxxxx\nxxxxxx\n");

        assert_eq!(result, expect);
    }

    #[test]
    #[should_panic]
    fn test_new5() {
        let mut mat = Matr2D::new(4, 4, Some('a'));
        let result = mat.get_str();
        let expect = String::from("aaa\naaa\naaa\naaaa\n");

        assert_eq!(result, expect);
    }

    #[test]
    #[should_panic]
    fn test_new6() {
        let mut mat = Matr2D::new(4, 4, None);
        let result = mat.get_str();
        let expect = String::from("xxx\nxxx\nxxx\nxxx\nxxx");

        assert_eq!(result, expect);
    }

    #[test]
    #[should_panic]
    fn test_new7() {
        let mat = Matr2D::new(0, 0, None);
    }

    #[test]
    fn test_fill() {
        let mut mat = Matr2D::new(5, 5, None);
        mat.fill('b');

        let result = mat.get_str();
        let expect = String::from("bbbb\nbbbb\nbbbb\nbbbb\nbbbb\n");

        assert_eq!(result, expect);
    }

    #[test]
    fn test_fill2() {
        let mut mat = Matr2D::new(6, 5, None);
        mat.fill(' ');

        let result = mat.get_str();
        let expect = String::from("     \n     \n     \n     \n     \n");

        assert_eq!(result, expect);
    }

    #[test]
    fn test_draw() {
        let mut mat = Matr2D::new(4, 4, None);
        mat.draw_point(1, 2, 'a');

        let result = mat.get_str();
        let expect = String::from(
            "xxx\nxxx\nxax\nxxx\n");

        assert_eq!(result, expect);
    }

    #[test]
    #[should_panic]
    fn test_draw2() {
        let mut mat = Matr2D::new(4, 4, None);
        mat.draw_point(1, 3, 'a');

        let result = mat.get_str();
        let expect = String::from(
            "xxx\nxxx\nxax\nxxx\n");

        assert_eq!(result, expect);
    }

    
    #[test]
    fn test_fill_draw() {
        let mut mat = Matr2D::new(6, 6, None);

        mat.draw_point(3, 4, 'z');
        mat.fill('c');

        let st1 = mat.get_str();
        let expect1 = String::from("ccccc\nccccc\nccccc\nccccc\nccccc\nccccc\n");

        mat.draw_point(0, 0, 'l');
        mat.draw_point(0, 0, 'b');
        mat.draw_point(1, 0, 'c');
        mat.draw_point(1, 1, 'a');

        let st2 = mat.get_str();
        let expect2 = String::from("bcccc\ncaccc\nccccc\nccccc\nccccc\nccccc\n");

        assert_eq!(st2, expect2);
    }
}





