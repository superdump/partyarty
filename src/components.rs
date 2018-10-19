use cgmath::Point3;
use specs::prelude::*;

use hitable::Hitable;
use material::Material;
use utils::point3;

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
    world.register::<Position>();
    world.register::<Hitable>();
    world.register::<Material>();
}
