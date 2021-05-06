use std::usize;

/// You probably will recognize that the width is lower than what is printed.
/// Well... this isn't a bug, what is happening is that '\n' is a hidden special char,
/// but it's a char so the width must include him 
use super::custom_vecs::*;

pub struct Matr {
    _max_width: usize,  // will be width - 1 because the last is \n
    _max_height: usize,
    _row_vec: ModernVec<AsciiVec>,
}

impl Matr {
    pub fn new(width: usize, height: usize, fill_with: Option<char>) -> Matr {
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
        Matr {_row_vec, _max_width: width-2, _max_height: height-1}
    }
    
    // We dont need to do so much error checks because the vecs already have a lot
    pub fn draw_point(&mut self, x: usize, y: usize, ch: char) {
        let row = &mut self._row_vec[y];
        row.replace(x, ch);
    }

    pub fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, ch: char) {
        let dx = x2 - x1;
        let dy = y2 - y1;

        for x in x1..x2 {
            let y = y1 + dy * (x - x1) / dx;
            self.draw_point(x, y, ch)
        }
    }
    
    pub fn fill(&mut self, ch: char) {
        for i in 0..self._row_vec.get_lock_len() {
            self._row_vec[i].fill(ch);
        }
    }

    pub fn get_str(&mut self) -> String {
        let mut rows: Vec<String> = Vec::new();

        for i in 0..self._row_vec.get_lock_len() {
            rows.push(self._row_vec[i].get_str());      // read error E0716 to see why String over &str in this case
        }

        let st: String = rows.concat();
        st
    }

    /* In mathematics, the slope or gradient of a line is a number
    that describes both the direction and the steepness of the line. Slope 
    is often denoted by the letter m. Slope is calculated by finding
    the ratio of the "vertical change" to the "horizontal change 
    (diferença)" between (any) two distinct points on a line. The 
    steepness, incline, or grade of a line is measured by the absolute
    value of the slope. A slope with greater absolute value indicates
    a steeper line. The direction of a line is either increasing, decrea-
    sing, horizontal or vertical
     - A line is increasing if it goes up from left to right. The slope
     is positive (m > 0) 
     - A line is decreasing if it goes down from left to right. The 
     slope is negative (m < 0) 
     - If a line is horizontal the slope is zero. This is a const function
     - If a line is vertical the slope is undefined*/
    //fn slope(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    //    (y2 - y1) / (x2 - x1)
    //}

    /* A y-intercept or vertical intercept is a point where the graph
    of a function intersects the y-axis of the coordinate system. As such,
    these pointes satisfy x = 0 */
    // m - slope
    //fn y_intercept(y1: usize, m: usize, x1: usize) -> usize {
    //    y1 - m * x1
    //}

}

// ---------------- TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut mat = Matr::new(4, 4, Some('a'));
        let result = mat.get_str();
        let expect = String::from("aaa\naaa\naaa\naaa\n");

        assert_eq!(result, expect);
    }

    #[test]
    fn test_new2() {
        let mut mat = Matr::new(4, 4, None);
        let result = mat.get_str();
        let expect = String::from("xxx\nxxx\nxxx\nxxx\n");

        assert_eq!(result, expect);
    }

    #[test]
    fn test_new3() {
        let mut mat = Matr::new(4, 7, None);
        let result = mat.get_str();
        let expect = String::from("xxx\nxxx\nxxx\nxxx\nxxx\nxxx\nxxx\n");

        assert_eq!(result, expect);
    }

    #[test]
    fn test_new4() {
        let mut mat = Matr::new(7, 4, None);
        let result = mat.get_str();
        let expect = String::from("xxxxxx\nxxxxxx\nxxxxxx\nxxxxxx\n");

        assert_eq!(result, expect);
    }

    #[test]
    #[should_panic]
    fn test_new5() {
        let mut mat = Matr::new(4, 4, Some('a'));
        let result = mat.get_str();
        let expect = String::from("aaa\naaa\naaa\naaaa\n");

        assert_eq!(result, expect);
    }

    #[test]
    #[should_panic]
    fn test_new6() {
        let mut mat = Matr::new(4, 4, None);
        let result = mat.get_str();
        let expect = String::from("xxx\nxxx\nxxx\nxxx\nxxx");

        assert_eq!(result, expect);
    }

    #[test]
    #[should_panic]
    fn test_new7() {
        Matr::new(0, 0, None);
    }

    #[test]
    fn test_fill() {
        let mut mat = Matr::new(5, 5, None);
        mat.fill('b');

        let result = mat.get_str();
        let expect = String::from("bbbb\nbbbb\nbbbb\nbbbb\nbbbb\n");

        assert_eq!(result, expect);
    }

    #[test]
    fn test_fill2() {
        let mut mat = Matr::new(6, 5, None);
        mat.fill(' ');

        let result = mat.get_str();
        let expect = String::from("     \n     \n     \n     \n     \n");

        assert_eq!(result, expect);
    }

    #[test]
    fn test_draw() {
        let mut mat = Matr::new(4, 4, None);
        mat.draw_point(1, 2, 'a');

        let result = mat.get_str();
        let expect = String::from(
            "xxx\nxxx\nxax\nxxx\n");

        assert_eq!(result, expect);
    }

    #[test]
    #[should_panic]
    fn test_draw2() {
        let mut mat = Matr::new(4, 4, None);
        mat.draw_point(1, 3, 'a');

        let result = mat.get_str();
        let expect = String::from(
            "xxx\nxxx\nxax\nxxx\n");

        assert_eq!(result, expect);
    }

    
    #[test]
    fn test_fill_draw() {
        let mut mat = Matr::new(6, 6, None);

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

        assert_eq!(st1, expect1);
        assert_eq!(st2, expect2);
    }
    /*
    #[test] 
    fn test_slope() {
        /*  Line P = (1,2) and Q = (13, 8) */
        let p = (1, 2);
        let q = (13, 8);
        /* calc for slope 
        m = (y2-y1)/(x2-x1)=(8-2)/(13-1)=6/12=1/2 */
        let r = Matr::slope(p.0, p.1, q.0, q.1);  
        assert_eq!(r, 1/2);

    }
    */
}







