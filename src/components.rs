use cgmath::{Point2, Point3};
use specs::prelude::*;

use color::Colorf32;
use hitable::Hitable;
use material::Material;
use ray::Ray;
use utils::point3;

pub struct PixelPosition(pub Point2<usize>);

pub fn pixel_position(x: usize, y: usize) -> PixelPosition {
    PixelPosition(Point2::new(x, y))
}

impl Component for PixelPosition {
    type Storage = VecStorage<Self>;
}

pub struct PixelColor(pub Colorf32);

pub fn pixel_color(r: f32, g: f32, b: f32, a: f32) -> PixelColor {
    PixelColor(Colorf32::new(r, g, b, a))
}

impl Component for PixelColor {
    type Storage = VecStorage<Self>;
}

pub struct SampleCount(pub f32);

impl Component for SampleCount {
    type Storage = VecStorage<Self>;
}

impl Component for Ray {
    type Storage = VecStorage<Self>;
}

pub struct Position(pub Point3<f32>);

pub fn position(x: f32, y: f32, z: f32) -> Position {
    Position(point3(x, y, z))
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl Component for Hitable {
    type Storage = VecStorage<Self>;
}

impl Component for Material {
    type Storage = VecStorage<Self>;
}

pub fn register_components(world: &mut World) {
    world.register::<PixelPosition>();
    world.register::<PixelColor>();
    world.register::<SampleCount>();
    world.register::<Ray>();
    world.register::<Position>();
    world.register::<Hitable>();
    world.register::<Material>();
}
