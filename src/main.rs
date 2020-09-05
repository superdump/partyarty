#[macro_use]
extern crate clap;
extern crate failure;
extern crate partyarty;
extern crate rand;
extern crate sdl2;

use clap::{App, Arg};
use failure::Error;
use partyarty::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;


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
        .arg(Arg::with_name("framerate")
            .short("f")
            .long("framerate")
            .value_name("FRAMERATE")
            .help("framerate of the preview")
            .takes_value(true))
        .get_matches();

    let width: usize = value_t!(matches.value_of("width"), usize).unwrap_or(640);
    let height: usize = value_t!(matches.value_of("height"), usize).unwrap_or(320);
    let samples: usize = value_t!(matches.value_of("samples"), usize).unwrap_or(0);
    let prefix: String = value_t!(matches.value_of("output"), String).unwrap_or(String::from(""));
    let scene: String = value_t!(matches.value_of("scene"), String).unwrap_or(String::from("random"));
    let framerate: f64 = value_t!(matches.value_of("framerate"), f64).unwrap_or(30.0f64);

    let buffer_output: Vec<u8> = vec![0; width * height * 4];

    let mut world = World::new();
    register_components(&mut world);

    let camera;
    let mut entities = match scene.as_ref() {
        "balls" => {
            let look_from = Vec3A::new(3.0, 3.0, 2.0);
            let look_at = Vec3A::new(0.0, 0.0, -1.0);
            camera = Camera::new(
                look_from,
                look_at,
                Vec3A::new(0.0, 1.0, 0.0),
                20.0,
                width as f32 / height as f32,
                2.0,
                (look_from - look_at).length(),
            );
            balls(&mut world)
        },
        "random" | _ => {
            camera = Camera::new(
                Vec3A::new(13.0, 2.0, 3.0),
                Vec3A::new(0.0, 0.0, 0.0),
                Vec3A::new(0.0, 1.0, 0.0),
                20.0,
                width as f32 / height as f32,
                0.1,
                10.0,
            );
            random_scene(&mut world)
        },
    };

    {
        let mut coords = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                coords.push((x, y));
            }
        }
        coords.shuffle(&mut thread_rng());
        let color = pixel_color(0.0, 0.0, 0.0, 0.0);
        let sample_count = SampleCount(0.0f32);
        for (x, y) in coords {
            entities.push(
                world.create_entity()
                    .with(pixel_position(x, y))
                    .with(color)
                    .with(sample_count)
                    .build()
            );
        }
    }

    world.insert(camera);
    world.insert(ImageFilePrefix(prefix));
    world.insert(Width(width));
    world.insert(Height(height));
    world.insert(Samples(samples));
    world.insert(FrameCount(0));
    world.insert(SamplesToProcessPerFrame(10000));
    world.insert(TargetFrameDuration(1.0f64 / framerate));
    world.insert(BufferOutput(buffer_output));
    world.insert(PixelsToProcess(BitSet::new()));


    let mut dispatcher = DispatcherBuilder::new()
        .with(PathTrace, "path_trace", &[])
        .with(SaveImage, "save_image", &["path_trace"])
        .build();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window(PKG_NAME, width as u32, height as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(
        Some(PixelFormatEnum::ABGR8888), width as u32, height as u32).unwrap();
    let rect = Some(Rect::new(0, 0, width as u32, height as u32));

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut samples_per_sec = SlidingAverage::default();
    let timers = Timers::default();
    world.insert(PerfTimers(timers));

    'mainloop: loop {
        timer_enter(&mut world, "frame");
        timer_enter(&mut world, "LOOP : events");
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'mainloop
                },
                _ => {}
            }
        }

        if samples > 0 {
            let frame_count = world.read_resource::<FrameCount>();
            if frame_count.0 > samples as u32 {
                break;
            }
        }

        timer_transition(&mut world, "LOOP : events", "LOOP : dispatch");
        dispatcher.dispatch(&mut world);
        world.maintain();

        timer_transition(&mut world, "LOOP : dispatch", "LOOP : update_frame");

        {
            world.exec(|(buffer, ): (Read<BufferOutput>, )| {
                texture.update(rect, &buffer.0, width * 4).unwrap();
            });
        }
        canvas.copy(&texture, rect, rect).unwrap();
        timer_exit(&mut world, "LOOP : update_frame");

        canvas.present();

        timer_exit(&mut world, "frame");
        timer_print(&mut world);
        let mut samples_per_sec_for_frame = 0.0;
        {
            world.exec(|(timers, samples_to_process,): (Read<PerfTimers>, Read<SamplesToProcessPerFrame>,)| {
                samples_per_sec_for_frame = samples_to_process.0 as f64 * 1000.0 / timers.0.frames_mean.q.back().unwrap();
            });
        }
        let mean = samples_per_sec.append(samples_per_sec_for_frame);
        {
            let frame_count = world.read_resource::<FrameCount>().0;
            if frame_count % 10 == 0 {
                println!(
                    "\tmean: {:.3} Msamples/s, frame: {:.3} Msamples/s",
                    mean / 1_000_000f64,
                    samples_per_sec_for_frame / 1_000_000f64,
                );
            }
        }
    }

    Ok(())
}
