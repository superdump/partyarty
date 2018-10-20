use cgmath::prelude::*;
use cgmath::vec3;
use image::{RGB, save_buffer};
use specs::prelude::*;

use camera::Camera;
use color::Colorf32;
use components::*;
use hitable::{hit, Hitable, HitRecord};
use material::{Material, scatter};
use ray::Ray;
use resources::*;
use utils::{lerp_vec3, random_float_01};

use std;

fn color<'a>(
    r: &Ray,
    position: &ReadStorage<'a, Position>,
    hitable: &ReadStorage<'a, Hitable>,
    material: &ReadStorage<'a, Material>,
    depth: u32,
) -> Colorf32 {
    use specs::Join;

    let mut closest_hit: Option<HitRecord> = None;
    let mut t_max = std::f32::MAX;
    for (position, hitable, material) in (position, hitable, material).join() {
        if let Some(mut rec) = hit(position, hitable, r, 0.001, t_max) {
            rec.material = Some(material.clone());
            if let Some(closest) = closest_hit {
                if rec.t < closest.t {
                    t_max = rec.t;
                    closest_hit = Some(rec);
                }
            } else {
                closest_hit = Some(rec);
            }
        }
    }
    if let Some(rec) = closest_hit {
        let scatter_option = scatter(r, &rec);
        if depth < 50 && scatter_option.is_some() {
            let (attenuation, scattered) = scatter_option.unwrap();
            return (attenuation * color(&scattered, position, hitable, material, depth + 1)).into();
        } else {
            return Colorf32::new(0.0, 0.0, 0.0, 1.0);
        }
    } else {
        let unit_direction = r.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        let c = lerp_vec3(vec3(1.0, 1.0, 1.0), t, vec3(0.5, 0.7, 1.0));
        return Colorf32::new(c.x, c.y, c.z, 1.0);
    }
}

pub struct RayCast;

impl<'a> System<'a> for RayCast {
    type SystemData = (
        ReadStorage<'a, PixelPosition>,
        Read<'a, Camera>,
        Read<'a, Width>,
        Read<'a, Height>,
        WriteStorage<'a, Ray>,
        // Write<'a, PerfTimers>,
    );

    fn run(
        &mut self,
        (
            pixel_positions,
            camera,
            width,
            height,
            mut rays,
            // mut timers,
        ): Self::SystemData,
    ) {
        use rayon::prelude::*;
        use specs::ParJoin;

        // let timers = &mut timers.0;
        // timers.enter("SYSTEM : RayCast");
        let width_f32 = width.0 as f32;
        let height_f32 = height.0 as f32;
        let height_minus_one = height.0 - 1;

        (&pixel_positions, &mut rays)
            .par_join()
            .for_each(|(pixel_position, ray)| {
                let x = pixel_position.0.x;
                let y = height_minus_one - pixel_position.0.y;
                let u = (x as f32 + random_float_01()) / width_f32;
                let v = (y as f32 + random_float_01()) / height_f32;
                *ray = camera.get_ray(u, v);
            });
        // timers.exit("SYSTEM : RayCast");
    }
}

pub struct PathTrace;

impl<'a> System<'a> for PathTrace {
    type SystemData = (
        ReadStorage<'a, Ray>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Hitable>,
        ReadStorage<'a, Material>,
        WriteStorage<'a, PixelColor>,
        // Write<'a, PerfTimers>,
    );

    fn run(
        &mut self,
        (
            rays,
            positions,
            hitables,
            materials,
            mut pixel_colors,
            // mut timers,
        ): Self::SystemData
    ) {
        use rayon::prelude::*;
        use specs::ParJoin;

        // let timers = &mut timers.0;
        // timers.enter("SYSTEM : PathTrace");

        (&rays, &mut pixel_colors)
            .par_join()
            .for_each(|(ray, pixel_color)| {
                pixel_color.0 += color(ray, &positions, &hitables, &materials, 0);
            });

        // timers.exit("SYSTEM : PathTrace");
    }
}

pub struct FrameAverage;

impl<'a> System<'a> for FrameAverage {
    type SystemData = (
        ReadStorage<'a, PixelPosition>,
        ReadStorage<'a, PixelColor>,
        Read<'a, Width>,
        Read<'a, FrameCount>,
        Write<'a, BufferOutput>,
        // Write<'a, PerfTimers>,
    );

    fn run(
        &mut self,
        (
            pixel_positions,
            pixel_colors,
            width,
            frame_count,
            mut buffer_output,
            // mut timers,
        ): Self::SystemData
    ) {
        use specs::Join;

        let width = width.0;
        let one_over_frame_count = 1.0 / frame_count.0 as f32;

        for (pixel_position, pixel_color) in (&pixel_positions, &pixel_colors).join() {
            let x = pixel_position.0.x;
            let y = pixel_position.0.y;
            let i = y * width + x;
            buffer_output.0[i] = (one_over_frame_count * pixel_color.0).into();
        }
    }
}

pub struct SaveImage;

impl<'a> System<'a> for SaveImage {
    type SystemData = (
        Read<'a, ImageFilePrefix>,
        Read<'a, Width>,
        Read<'a, Height>,
        Read<'a, Samples>,
        Read<'a, FrameCount>,
        Read<'a, BufferOutput>,
        // Write<'a, PerfTimers>,
    );

    fn run(
        &mut self,
        (
            prefix,
            width,
            height,
            samples,
            frame_count,
            buffer_output,
            // mut timers,
        ): Self::SystemData
    ) {
        let prefix = &prefix.0;
        let samples = samples.0;
        let frame_count = frame_count.0;
        if frame_count > samples as u32 || prefix.len() < 1 {
            return;
        }

        // let timers = &mut timers.0;
        // timers.enter("SYSTEM : SaveImage");

        let width = width.0;
        let height = height.0;
        let buffer = &buffer_output.0;
        let mut image_buffer = vec![0u8; width * height * 3];
        for i in 0..(width * height) {
            let px = buffer[i];
            image_buffer[i * 3 + 0] = ((px >> 16) & 0xff) as u8;
            image_buffer[i * 3 + 1] = ((px >> 8) & 0xff) as u8;
            image_buffer[i * 3 + 2] = ((px >> 0) & 0xff) as u8;
        }
        let filename = format!("{}{:05}.png", prefix, frame_count);
        save_buffer(filename, &image_buffer, width as u32, height as u32, RGB(8)).unwrap();
        // timers.exit("SYSTEM : SaveImage");
    }
}
