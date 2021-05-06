use crate::*;
use std::time::Duration;
use std::thread;

const THREAD_DELAY: f32 = 0.1;

pub struct Renderer {
    pub matr: Matr
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Renderer {
        Renderer {
            matr: Matr::new(width, height, Some('-'))
        }
    }
    fn start_loop(&mut self) -> ! {
        loop {
            self.matr.fill('-');
            thread::sleep(Duration::from_secs_f32(THREAD_DELAY));            
            
            self.matr.draw_line(3.0, 3.0, 5.0, 6.0, 'x');
        }
    }
    fn end_loop() {}
}


