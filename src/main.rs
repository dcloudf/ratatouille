use core::f64;
use std::fs::File;
use std::io::{self, BufWriter};
use std::rc::Rc;

use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::vec3::Vec3;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;

fn main() -> io::Result<()> {
    let mut world = HittableList::default();
    let r = f64::cos(f64::consts::PI / 4.);
    let material_left = Rc::new(Lambertian::new(Vec3::new(0., 0., 1.)));
    let material_right = Rc::new(Lambertian::new(Vec3::new(1., 0., 0.)));

    world.add(Rc::new(Sphere::new(
        Vec3::new(-r, 0., -1.),
        r,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(r, 0., -1.),
        r,
        material_right,
    )));

    let camera = Camera::new(16.0 / 9.0, 400, 100, 50, 90.);
    let file = File::create("image.ppm")?;
    let mut writer = BufWriter::new(file);
    camera.render(&world, &mut writer)
}
