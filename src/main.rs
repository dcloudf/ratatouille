use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::rc::Rc;

use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod interval;
pub mod ray;
pub mod sphere;
pub mod vec3;

fn main() -> io::Result<()> {
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Vec3::new(0f64, 0f64, -1f64), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0f64, -100.5, -1f64), 100.)));

    let camera = Camera::new(16.0 / 9.0, 400);
    let file = File::create("image.ppm")?;
    let mut writer = BufWriter::new(file);
    camera.render(&world, &mut writer)
}
