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
        .arg(Arg::with_name("samples")
            .short("s")
            .long("samples")
            .value_name("SAMPLES")
            .help("Samples (rays) per pixel")
            .takes_value(true))
        .get_matches();

    let width: usize = value_t!(matches.value_of("width"), usize).unwrap_or(640);
    let height: usize = value_t!(matches.value_of("height"), usize).unwrap_or(320);
    let samples: usize = value_t!(matches.value_of("samples"), usize).unwrap_or(100);

    let buffer: Vec<u32> = vec![0; width * height];
    let camera = Camera::new();

    let mut world = World::new();
    register_components(&mut world);

    world.add_resource(camera);
    world.add_resource(Width(width));
    world.add_resource(Height(height));
    world.add_resource(Samples(samples));
    world.add_resource(FrameCount(0));
    world.add_resource(Buffer(buffer));

    let mut entities = Vec::<Entity>::new();
    entities.push(
        world.create_entity()
            .with(Position(Point3::new(0.0, 0.0, -1.0)))
            .with(Hitable::Sphere(Sphere(0.5)))
            .build()
    );

    let mut dispatcher = DispatcherBuilder::new()
        .with(PathTrace, "path_trace", &[])
        .build();

    let mut window = Window::new(PKG_NAME, width, height, WindowOptions::default())?;

    while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        {
            let mut frame_count = world.write_resource::<FrameCount>();
            (*frame_count).0 += 1;
        }

        dispatcher.dispatch(&mut world.res);
        world.maintain();

        let buffer = world.read_resource::<Buffer>();
        window.update_with_buffer(&buffer.0)?;
    }

    Ok(())
}
