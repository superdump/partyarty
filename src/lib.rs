extern crate cgmath;

pub use cgmath::prelude::*;
pub use cgmath::*;

mod color;
mod hitable;
mod ray;

pub use color::*;
pub use hitable::*;
pub use ray::*;
