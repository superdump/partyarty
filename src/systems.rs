use cgmath::prelude::*;
use cgmath::vec3;
use image::{RGB, save_buffer};
use specs::prelude::*;

use camera::Camera;
use color::Colorf32;
use components::Position;
use hitable::{hit, Hitable, HitRecord};
use ray::Ray;
use resources::*;
use utils::{lerp_vec3, random_float_01, random_in_unit_sphere};

use std;

fn color<'a>(r: &Ray, position: &ReadStorage<'a, Position>, hitable: &ReadStorage<'a, Hitable>, state: &mut u32) -> Colorf32 {
    use specs::Join;

    let mut closest_hit: Option<HitRecord> = None;
    let mut t_max = std::f32::MAX;
    for (position, hitable) in (position, hitable).join() {
        if let Some(rec) = hit(position, hitable, &r, 0.001, t_max) {
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
        let target = rec.p + rec.normal + random_in_unit_sphere(state);
        return (0.5 * color(&Ray::new(rec.p, target - rec.p), position, hitable, state)).into();
    } else {
        let unit_direction = r.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        let c = lerp_vec3(vec3(1.0, 1.0, 1.0), t, vec3(0.5, 0.7, 1.0));
        return Colorf32::new(c.x, c.y, c.z, 1.0);
    }
}

pub struct PathTrace;

impl<'a> System<'a> for PathTrace {
    type SystemData = (
        Read<'a, Camera>,
        Read<'a, Width>,
        Read<'a, Height>,
        Read<'a, FrameCount>,
        Write<'a, BufferTotals>,
        Write<'a, BufferOutput>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Hitable>,
    );

    fn run(&mut self, (camera, width, height, frame_count, mut buffer_totals, mut buffer_output, position, hitable): Self::SystemData) {
        let width = width.0;
        let height = height.0;
        let frame_count = frame_count.0;
        let totals = &mut buffer_totals.0;
        let buffer = &mut buffer_output.0;

        let mut i = 0;
        for y in (0..height).rev() {
            let mut state = (y as u32 * 9781 + frame_count * 6271) | 1;
            for x in 0..width {
                let u = (x as f32 + random_float_01(&mut state)) / (width as f32);
                let v = (y as f32 + random_float_01(&mut state)) / (height as f32);
                let r = camera.get_ray(u, v);
                totals[i] += color(&r, &position, &hitable, &mut state);
                buffer[i] = (totals[i] / frame_count as f32).into();
                i += 1;
            }
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
    );

    fn run(&mut self, (prefix, width, height, samples, frame_count, buffer_output): Self::SystemData) {
        let prefix = &prefix.0;
        let width = width.0;
        let height = height.0;
        let samples = samples.0;
        let frame_count = frame_count.0;
        if frame_count > samples as u32 || prefix.len() < 1 {
            return;
        }
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
    }
}
