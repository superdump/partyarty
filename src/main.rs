extern crate failure;
extern crate minifb;

use failure::Error;
use minifb::{Key, KeyRepeat, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() -> Result<(), Error> {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("ArtyArty", WIDTH, HEIGHT, WindowOptions::default())?;

    while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        for i in buffer.iter_mut() {
            *i = 0;
        }

        window.update_with_buffer(&buffer)?;
    }

    Ok(())
}
