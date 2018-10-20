use cgmath::vec3;
use cgmath::prelude::*;
use specs::{Entity, World};
use specs::prelude::*;

use components::position;
use hitable::sphere;
use material::{dielectric, lambertian, metal};
use utils::{point3, random_float_01};

pub fn balls(world: &mut World) -> Vec<Entity> {
    let mut entities = Vec::<Entity>::new();
    entities.push(
        world.create_entity()
            .with(position(0.0, 0.0, -1.0))
            .with(sphere(0.5))
            .with(lambertian(vec3(0.1, 0.2, 0.5)))
            .build()
    );
    entities.push(
        world.create_entity()
            .with(position(0.0, -100.5, -1.0))
            .with(sphere(100.0))
            .with(lambertian(vec3(0.8, 0.8, 0.0)))
            .build()
    );
    entities.push(
        world.create_entity()
            .with(position(1.0, 0.0, -1.0))
            .with(sphere(0.5))
            .with(metal(vec3(0.8, 0.6, 0.2), 0.0))
            .build()
    );
    entities.push(
        world.create_entity()
            .with(position(-1.0, 0.0, -1.0))
            .with(sphere(0.5))
            .with(dielectric(1.5))
            .build()
    );
    entities.push(
        world.create_entity()
            .with(position(-1.0, 0.0, -1.0))
            .with(sphere(-0.45))
            .with(dielectric(1.5))
            .build()
    );
    entities
}

pub fn random_scene(world: &mut World) -> Vec<Entity> {
    let mut entities = Vec::<Entity>::new();
    entities.push(
        world.create_entity()
            .with(position(0.0, -1000.0, 0.0))
            .with(sphere(1000.0))
            .with(lambertian(vec3(0.5, 0.5, 0.5)))
            .build()
    );
    for a in -11..11 {
        for b in -11..11 {
            let center = point3(
                a as f32 + 0.9 * random_float_01(),
                0.2,
                b as f32 + 0.9 * random_float_01()
            );
            if (center - point3(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                let m = random_float_01();
                if m < 0.8 {
                    entities.push(
                        world.create_entity()
                            .with(position(center.x, center.y, center.z))
                            .with(sphere(0.2))
                            .with(lambertian(vec3(
                                random_float_01() * random_float_01(),
                                random_float_01() * random_float_01(),
                                random_float_01() * random_float_01(),
                            )))
                            .build()
                    );
                } else if m < 0.95 {
                    entities.push(
                        world.create_entity()
                            .with(position(center.x, center.y, center.z))
                            .with(sphere(0.2))
                            .with(metal(
                                vec3(
                                    0.5 * (1.0 + random_float_01()),
                                    0.5 * (1.0 + random_float_01()),
                                    0.5 * (1.0 + random_float_01()),
                                ),
                                0.5 * random_float_01(),
                            ))
                            .build()
                    );
                } else {
                    entities.push(
                        world.create_entity()
                            .with(position(center.x, center.y, center.z))
                            .with(sphere(0.2))
                            .with(dielectric(1.5))
                            .build()
                    );
                }
            }
        }
    }
    entities.push(
        world.create_entity()
            .with(position(0.0, 1.0, 0.0))
            .with(sphere(1.0))
            .with(dielectric(1.5))
            .build()
    );
    entities.push(
        world.create_entity()
            .with(position(-4.0, 1.0, 0.0))
            .with(sphere(1.0))
            .with(lambertian(vec3(0.4, 0.2, 0.1)))
            .build()
    );
    entities.push(
        world.create_entity()
            .with(position(4.0, 1.0, 0.0))
            .with(sphere(1.0))
            .with(metal(vec3(0.7, 0.6, 0.5), 0.0))
            .build()
    );
    entities
}
