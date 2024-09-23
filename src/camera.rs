use std::fs::File;
use std::io::{self, BufWriter, Write};

use rand::prelude::*;

use crate::color::write_color;
use crate::{hittable::Hittable, interval::Interval, ray::Ray, vec3::Vec3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u64,
    pub samples_per_pixel: u64,
    pub max_depth: i64,
    pub vfov: f64,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    image_height: i64,
    pixel_samples_scale: f64,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u64,
        samples_per_pixel: u64,
        max_depth: i64,
        vfov: f64,
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
    ) -> Self {
        let image_height = match image_width as f64 / aspect_ratio {
            ..=1f64 => 1,
            n => n as i64,
        };
        let pixel_samples_scale = 1. / samples_per_pixel as f64;

        let focal_length = (lookfrom - lookat).len();
        let h = f64::tan(f64::to_radians(vfov) / 2.);
        let viewport_height = 2. * h * focal_length;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let center = lookfrom;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        let pixel_delta_u = viewport_u.clone() / image_width as f64;
        let pixel_delta_v = viewport_v.clone() / image_height as f64;

        let viewport_upper_left = center.clone()
            - (w * focal_length)
            - (viewport_u.clone() / 2.)
            - (viewport_v.clone() / 2.);
        let pixel00_loc =
            viewport_upper_left.clone() + (pixel_delta_u.clone() + pixel_delta_v.clone()) * 0.5;

        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            image_height,
            pixel_samples_scale,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            lookfrom,
            lookat,
            vup,
        }
    }
    pub fn render(&self, world: &dyn Hittable, mut writer: &mut BufWriter<File>) -> io::Result<()> {
        writer.write_all(
            format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes(),
        )?;
        for j in 0..self.image_height {
            println!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Vec3::new(0., 0., 0.);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i as i64, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }
                write_color(&mut writer, pixel_color * self.pixel_samples_scale)?;
            }
        }
        Ok(())
    }

    fn get_ray(&self, i: i64, j: i64) -> Ray {
        let offset = self.sample_square();
        let pixel_center = self.pixel00_loc.clone()
            + (self.pixel_delta_u.clone() * (i as f64 + offset.x()) as f64)
            + (self.pixel_delta_v.clone() * (j as f64 + offset.y()) as f64);
        let ray_direction = pixel_center.clone() - self.center.clone();
        Ray::new(self.center, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        let mut rng = thread_rng();
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        Vec3::new(x - 0.5, y - 0.5, 0.)
    }

    fn ray_color(&self, r: &Ray, depth: i64, world: &dyn Hittable) -> Vec3 {
        if depth <= 0 {
            return Vec3::new(0., 0., 0.);
        }
        if let Some(rec) = world.hit(r, Interval::new(0.001, f64::MAX)) {
            let result = rec.mat.scatter(r, &rec);
            if result.is_scattered {
                return result.attenuation * self.ray_color(&result.scattered, depth - 1, world);
            }
            return Vec3::default();
        }

        let unit_direction = r.direction.unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
    }
}
