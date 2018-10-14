#[macro_use]
extern crate clap;
extern crate failure;
extern crate minifb;
extern crate partyarty;

use clap::{App, Arg};
use failure::Error;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use partyarty::*;


const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const PKG_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const PKG_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn lerp(a: Vector3<f32>, t: f32, b: Vector3<f32>) -> Vector3<f32> {
    (1.0 - t) * a + t * b
}

fn color(r: &Ray) -> Colorf32 {
    let center = Point3::new(0.0, 0.0, -1.0);
    let s = Sphere::new(center, 0.5);
    if let Some(rec) = hit(&Hitable::Sphere(s), r, 0.0, std::f32::MAX) {
        return (0.5 * (rec.normal + vec3(1.0, 1.0, 1.0))).into();
    }
    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    let c = lerp(vec3(1.0, 1.0, 1.0), t, vec3(0.5, 0.7, 1.0));
    Colorf32::new(c.x, c.y, c.z, 1.0)
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
    let height = value_t!(matches.value_of("height"), usize).unwrap_or(320);
    let mut buffer: Vec<u32> = vec![0; width * height];

    let mut window = Window::new(PKG_NAME, width, height, WindowOptions::default())?;

    while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        let mut i = 0;
        let lower_left_corner = Point3::new(-2.0, -1.0, -1.0);
        let horizontal = vec3(4.0, 0.0, 0.0);
        let vertical = vec3(0.0, 2.0, 0.0);
        let origin = Point3::new(0.0, 0.0, 0.0);
        for y in (0..height).rev() {
            for x in 0..width {
                let u = (x as f32) / (width as f32);
                let v = (y as f32) / (height as f32);
                let r = Ray::new(origin, (lower_left_corner + u * horizontal + v * vertical) - origin);
                let col = color(&r);
                buffer[i] = col.into();
                i += 1;
            }
        }

        window.update_with_buffer(&buffer)?;
    }

    Ok(())
}
