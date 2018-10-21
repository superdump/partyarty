use cgmath::prelude::*;
use cgmath::vec3;
use image::{RGBA, save_buffer};
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
        Write<'a, PerfTimers>,
    );

    fn run(
        &mut self,
        (
            pixel_positions,
            camera,
            width,
            height,
            mut rays,
            mut timers,
        ): Self::SystemData,
    ) {
        use rayon::prelude::*;
        use specs::ParJoin;

        let timers = &mut timers.0;
        timers.enter("SYSTEM : RayCast");
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
        timers.exit("SYSTEM : RayCast");
    }
}

pub struct PathTrace;

impl<'a> System<'a> for PathTrace {
    type SystemData = (
        ReadStorage<'a, Ray>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Hitable>,
        ReadStorage<'a, Material>,
        Read<'a, TargetFrameDuration>,
        WriteStorage<'a, PixelColor>,
        WriteStorage<'a, SampleCount>,
        Write<'a, SamplesToProcessPerFrame>,
        Write<'a, PixelsToProcess>,
        Write<'a, PerfTimers>,
    );

    fn run(
        &mut self,
        (
            rays,
            positions,
            hitables,
            materials,
            target_frame_duration,
            mut pixel_colors,
            mut sample_counts,
            mut samples_to_process,
            mut pixels_to_process,
            mut timers,
        ): Self::SystemData
    ) {
        use rayon::prelude::*;
        use specs::{Join, ParJoin};

        let timers = &mut timers.0;
        timers.enter("SYSTEM : PathTrace");

        let target_frame_duration = target_frame_duration.0;
        let actual_frame_duration;
        if let Some(d) = timers.frames_mean.q.back() {
            actual_frame_duration = *d / 1000.0;
        } else {
            actual_frame_duration = target_frame_duration;
        }
        let samples_to_process = &mut samples_to_process.0;
        let new_samples_to_process =
            (*samples_to_process *
                ((1000.0 * (0.99 * target_frame_duration) / actual_frame_duration) as u64)
            ) / 1000;
        *samples_to_process = new_samples_to_process;

        let pixels_to_process = &mut pixels_to_process.0;
        let mut pixels_to_process_now = BitSet::new();
        let mut count = 0;
        let mut pixel_collection: BitSet = (&*pixels_to_process)
            .join()
            .take_while(|_| { count += 1; count < new_samples_to_process })
            .collect();
        pixels_to_process_now |= &pixel_collection;
        *pixels_to_process &= &!pixels_to_process_now.clone();
        if count < new_samples_to_process {
            *pixels_to_process = rays.mask().clone();
            *pixels_to_process &= pixel_colors.mask();
            pixel_collection = (&*pixels_to_process)
                .join()
                .take_while(|_| { count += 1; count < new_samples_to_process })
                .collect();
            pixels_to_process_now |= &pixel_collection;
        }

        (&rays, &mut pixel_colors, &mut sample_counts, pixels_to_process_now.clone())
            .par_join()
            .for_each(|(ray, pixel_color, sample_count, _)| {
                pixel_color.0 += color(ray, &positions, &hitables, &materials, 0);
                sample_count.0 += 1.0;
            });

        timers.exit("SYSTEM : PathTrace");
    }
}

pub struct SampleAverage;

impl<'a> System<'a> for SampleAverage {
    type SystemData = (
        ReadStorage<'a, PixelPosition>,
        ReadStorage<'a, PixelColor>,
        ReadStorage<'a, SampleCount>,
        Read<'a, Width>,
        Write<'a, BufferOutput>,
        Write<'a, PerfTimers>,
    );

    fn run(
        &mut self,
        (
            pixel_positions,
            pixel_colors,
            sample_counts,
            width,
            mut buffer_output,
            mut timers,
        ): Self::SystemData
    ) {
        use specs::Join;

        let timers = &mut timers.0;
        timers.enter("SYSTEM : SampleAverage");
        let width = width.0;
        let buffer = &mut buffer_output.0;

        for (pixel_position, pixel_color, sample_count) in (&pixel_positions, &pixel_colors, &sample_counts).join() {
            let x = pixel_position.0.x;
            let y = pixel_position.0.y;
            let i = y * width + x;
            let (a, r, g, b) = (pixel_color.0 / sample_count.0).as_argb8888();
            buffer[i * 4 + 0] = r;
            buffer[i * 4 + 1] = g;
            buffer[i * 4 + 2] = b;
            buffer[i * 4 + 3] = a;
        }
        timers.exit("SYSTEM : SampleAverage");
    }
}

pub struct SaveImage;

impl<'a> System<'a> for SaveImage {
    type SystemData = (
        Read<'a, ImageFilePrefix>,
        Read<'a, Width>,
        Read<'a, Height>,
        Read<'a, Samples>,
        Read<'a, BufferOutput>,
        Write<'a, FrameCount>,
        Write<'a, PerfTimers>,
    );

    fn run(
        &mut self,
        (
            prefix,
            width,
            height,
            samples,
            buffer_output,
            mut frame_count,
            mut timers,
        ): Self::SystemData
    ) {
        let prefix = &prefix.0;
        let samples = samples.0;
        frame_count.0 += 1;
        let frame_count = frame_count.0;
        if samples < 1 || frame_count > samples as u32 || prefix.len() < 1 {
            return;
        }

        let timers = &mut timers.0;
        timers.enter("SYSTEM : SaveImage");

        let width = width.0;
        let height = height.0;
        let buffer = &buffer_output.0;
        let filename = format!("{}{:05}.png", prefix, frame_count);
        save_buffer(filename, buffer, width as u32, height as u32, RGBA(8)).unwrap();
        timers.exit("SYSTEM : SaveImage");
    }
}
