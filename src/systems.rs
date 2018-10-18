use cgmath::prelude::*;
use cgmath::vec3;
use specs::prelude::*;

use camera::Camera;
use color::Colorf32;
use components::Position;
use hitable::{hit, Hitable, HitRecord};
use resources::*;
use utils::{lerp_vec3, random_float_01};

use std;

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
        use specs::Join;

        let width = width.0;
        let height = height.0;
        let frame_count = frame_count.0;
        let totals = &mut buffer_totals.0;
        let buffer = &mut buffer_output.0;

        let mut i = 0;
        for y in (0..height).rev() {
            let mut state = (y as u32 * 9781 + frame_count * 6271) | 1;
            for x in 0..width {
                let col: Colorf32;
                let u = (x as f32 + random_float_01(&mut state)) / (width as f32);
                let v = (y as f32 + random_float_01(&mut state)) / (height as f32);
                let r = camera.get_ray(u, v);
                let mut closest_hit: Option<HitRecord> = None;
                let mut t_max = std::f32::MAX;
                for (position, hitable) in (&position, &hitable).join() {
                    if let Some(rec) = hit(position, hitable, &r, 0.0, t_max) {
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
                    col = (0.5 * (rec.normal + vec3(1.0, 1.0, 1.0))).into();
                } else {
                    let unit_direction = r.direction.normalize();
                    let t = 0.5 * (unit_direction.y + 1.0);
                    let c = lerp_vec3(vec3(1.0, 1.0, 1.0), t, vec3(0.5, 0.7, 1.0));
                    col = Colorf32::new(c.x, c.y, c.z, 1.0);
                }
                totals[i] += col;
                buffer[i] = (totals[i] / frame_count as f32).into();
                i += 1;
            }
        }
    }
}
