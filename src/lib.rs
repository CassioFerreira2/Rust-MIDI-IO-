use vecs::traits::*;
use vecs::ModernVec;

struct Matr1D {
    _vec: ModernVec<char>,
}

impl Matr1D {
    fn new(len: usize) -> Matr1D {
        let _vec = ModernVec::new();
        _vec.lock(len);

        
    }

    fn safe_fill_with_break(&self, char: char) {
        for i in 0..self._vec.get_lock_len() {
            self._vec.replace(i, char);
            if i == self._vec.get_lock_len() {
                println!("a");
            }
        }

    } 
}

impl Matr1D {

}

struct Matr2D {
    _row_vec: ModernVec<char>,
    _char_vec: ModernVec<char>
}

impl Matr2D {
    fn new() -> Mat2D {
        Matr2D {
            _row_vec: ModernVec
        }
    }

    fn get()
}

pub struct Renderer {
    _drawable: Vec<Vec<char>>,
    _width: usize, 
    _height: usize
}

impl Renderer {
    
    pub fn new() -> Renderer {

        let _width: usize = 12;
        let _height: usize = 6;

        let _rows = ModernVec::new();
        let _chars = ModernVec::new();

        _rows.lock(_height);
        _chars.lock(_width);
        


        Renderer {_drawable, _width, _height}
    }

    /// Draw will draw a character in the given x and y axis
    /// the character will be replaced by mem::replace 
    pub fn draw(&mut self, ch: char, x: usize, y: usize) {
        let pos = y*self._width + x;
        println!("pos: {}", pos);
        
     }

    /// Render will create a new vector, this tima a 2d vector
    /// by adding \n at the end of each line
    pub fn render(&self) {
        let mut new_vec = self._drawable.clone();
        
        for l in (self._width..self._width*self._height).step_by(self._width) {
            new_vec.insert(l, '\n')
        }
        let strin: String = new_vec.iter().collect();
        println!("{}", strin);
    }

    /// Clear the terminal screen
    pub fn clear(&self) {
        print!("\x1B[2J");
    }
}
 




















