use cgmath::Point3;
use specs::prelude::*;

use hitable::Hitable;
use material::Material;

pub struct Position(pub Point3<f32>);

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
