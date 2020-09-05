use glam::Vec3A;
use specs::prelude::*;

use color::Colorf32;
use hitable::Hitable;
use material::Material;

pub struct Vec2u {
    pub x: usize,
    pub y: usize,
}

impl Vec2u {
    pub fn new(x: usize, y: usize) -> Self {
        Vec2u{ x, y }
    }
}
pub struct PixelPosition(pub Vec2u);

pub fn pixel_position(x: usize, y: usize) -> PixelPosition {
    PixelPosition(Vec2u::new(x, y))
}

impl Component for PixelPosition {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Copy)]
pub struct PixelColor(pub Colorf32);

pub fn pixel_color(r: f32, g: f32, b: f32, a: f32) -> PixelColor {
    PixelColor(Colorf32::new(r, g, b, a))
}

impl Component for PixelColor {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Copy)]
pub struct SampleCount(pub f32);

impl Component for SampleCount {
    type Storage = VecStorage<Self>;
}

pub struct Position(pub Vec3A);

pub fn position(x: f32, y: f32, z: f32) -> Position {
    Position(Vec3A::new(x, y, z))
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
    world.register::<Position>();
    world.register::<Hitable>();
    world.register::<Material>();
}
