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
    let material_ground = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.)));
    let material_center = Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_bubble = Rc::new(Dielectric::new(1. / 1.5));
    let material_right = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    world.add(Rc::new(Sphere::new(
        Vec3::new(0., -100.5, -1.),
        100.,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0., 0., -1.2),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(-1., 0., -1.),
        0.5,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(-1., 0., -1.),
        0.4,
        material_bubble,
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(1., 0., -1.),
        0.5,
        material_right,
    )));

    let camera = Camera::new(16.0 / 9.0, 400, 100, 50);
    let file = File::create("image.ppm")?;
    let mut writer = BufWriter::new(file);
    camera.render(&world, &mut writer)
}
