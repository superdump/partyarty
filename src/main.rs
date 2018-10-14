extern crate cgmath;
#[macro_use]
extern crate clap;
extern crate failure;
extern crate minifb;

use cgmath::{Point3};
use clap::{App, Arg};
use failure::Error;
use minifb::{Key, KeyRepeat, Window, WindowOptions};

const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const PKG_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const PKG_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn rgba_as_u32(r: u8, g: u8, b: u8, a: u8) -> u32 {
    ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

fn main() -> Result<(), Error> {
    let matches = App::new(PKG_NAME)
        .version(PKG_VERSION)
        .author(PKG_AUTHORS)
        .about(PKG_DESCRIPTION)
        .arg(Arg::with_name("width")
            .short("w")
            .long("width")
            .value_name("WIDTH")
            .help("Rendered / displayed width")
            .takes_value(true))
        .arg(Arg::with_name("height")
            .short("h")
            .long("height")
            .value_name("HEIGHT")
            .help("Rendered / displayed height")
            .takes_value(true))
        .get_matches();

    let width = value_t!(matches.value_of("width"), usize).unwrap_or(640);
    let height = value_t!(matches.value_of("height"), usize).unwrap_or(480);
    let mut buffer: Vec<u32> = vec![0; width * height];

    let mut window = Window::new(PKG_NAME, width, height, WindowOptions::default())?;

    while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        let mut i = 0;
        for y in (0..height).rev() {
            for x in 0..width {
                let col = Point3::new(
                    (x as f32) / (width as f32),
                    (y as f32) / (height as f32),
                    0.2f32,
                );
                let icol = Point3::new(
                    (255.99f32 * col.x) as u8,
                    (255.99f32 * col.y) as u8,
                    (255.99f32 * col.z) as u8,
                );
                buffer[i] = rgba_as_u32(icol.x, icol.y, icol.z, 255u8);
                i += 1;
            }
        }

        window.update_with_buffer(&buffer)?;
    }

    Ok(())
}
