extern crate cgmath;

pub use cgmath::prelude::*;
pub use cgmath::*;

mod color;
mod hitable;
mod ray;
mod utils;

pub use color::*;
pub use hitable::*;
pub use ray::*;
pub use utils::*;
