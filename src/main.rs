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
            .help("Samples (rays) per pixel to stop writing images")
            .takes_value(true))
        .arg(Arg::with_name("scene")
            .short("c")
            .long("scene")
            .value_name("SCENE")
            .help("The scene to render")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("OUTPUT")
            .help("output image file name prefix")
            .takes_value(true))
        .get_matches();

    let width: usize = value_t!(matches.value_of("width"), usize).unwrap_or(640);
    let height: usize = value_t!(matches.value_of("height"), usize).unwrap_or(320);
    let samples: usize = value_t!(matches.value_of("samples"), usize).unwrap_or(100);
    let prefix: String = value_t!(matches.value_of("output"), String).unwrap_or(String::from(""));
    let scene: String = value_t!(matches.value_of("scene"), String).unwrap_or(String::from("random"));

    let buffer_totals: Vec<Colorf32> = vec![Colorf32::new(0.0, 0.0, 0.0, 0.0); width * height];
    let buffer_output: Vec<u32> = vec![0; width * height];

    let mut world = World::new();
    register_components(&mut world);

    let camera;
    let mut entities = match scene.as_ref() {
        "balls" => {
            camera = Camera::new(
                point3(-2.0, 2.0, 1.0),
                point3(0.0, 0.0, -1.0),
                vec3(0.0, 1.0, 0.0),
                20.0,
                width as f32 / height as f32,
            );
            balls(&mut world)
        },
        "random" | _ => {
            camera = Camera::new(
                point3(13.0, 2.0, 3.0),
                point3(0.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
                20.0,
                width as f32 / height as f32,
            );
            random_scene(&mut world)
        },
    };

    for y in 0..height {
        for x in 0..width {
            entities.push(
                world.create_entity()
                    .with(pixel_position(x, y))
                    .with(pixel_color(0.0, 0.0, 0.0, 0.0))
                    .with(Ray::default())
                    .build()
            );
        }
    }

    world.add_resource(camera);
    world.add_resource(ImageFilePrefix(prefix));
    world.add_resource(Width(width));
    world.add_resource(Height(height));
    world.add_resource(Samples(samples));
    world.add_resource(FrameCount(0));
    world.add_resource(BufferOutput(buffer_output));


    let mut dispatcher = DispatcherBuilder::new()
        .with(RayCast, "ray_cast", &[])
        .with(PathTrace, "path_trace", &["ray_cast"])
        .with(FrameAverage, "frame_average", &["path_trace"])
        .with(SaveImage, "save_image", &["path_trace"])
        .build();

    let mut window = Window::new(PKG_NAME, width, height, WindowOptions::default())?;

    let timers = Timers::default();
    world.add_resource(PerfTimers(timers));

    while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        timer_enter(&world, "frame");
        {
            let mut frame_count = world.write_resource::<FrameCount>();
            frame_count.0 += 1;
            if frame_count.0 > samples as u32 {
                break;
            }
        }

        timer_enter(&world, "LOOP : dispatch");
        dispatcher.dispatch(&mut world.res);
        world.maintain();

        timer_transition(&world, "LOOP : dispatch", "LOOP : update_frame");

        let buffer = &world.read_resource::<BufferOutput>().0;
        window.update_with_buffer(buffer)?;
        timer_exit(&world, "LOOP : update_frame");

        timer_exit(&world, "frame");
        timer_print(&world);
    }

    Ok(())
}
