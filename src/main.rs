pub mod lib;

use lib::Renderer;
use std::thread;
use std::time::Duration;

fn main() {
    let mut rend = Renderer::new();
    rend.draw('a', 0, 1);
    rend.render();
}
