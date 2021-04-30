use std::{io, ops::Deref};
use std::mem;



pub struct Renderer {
    _drawable: Vec<Vec<char>>,
    _width: usize, 
    _height: usize
}

impl Renderer {
    
    pub fn new() -> Renderer {

        let _width: usize = 12;
        let _height: usize = 6;

        let _line_vec = Vec::with_capacity(_height);
        let _char_vec = Vec::with_capacity(_width);
        


        let _vec =  

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
 




















