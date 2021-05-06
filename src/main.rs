pub mod vecs;
pub mod rend;

pub use vecs::*;
pub use vecs::matr::*;
pub use rend::renderer::*;

use std::thread;
use std::time::Duration;

fn main() {
    let rend = Renderer::new(70, 25);
}





