use std::fs::File;
use std::io::{self, BufWriter, Write};

use crate::color::write_color;
use crate::{hittable::Hittable, interval::Interval, ray::Ray, vec3::Vec3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u64,
    image_height: i64,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u64) -> Self {
        let image_height = match image_width as f64 / aspect_ratio {
            ..=1f64 => 1,
            n => n as i64,
        };
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let center = Vec3::new(0f64, 0f64, 0f64);

        let viewport_u = Vec3::new(viewport_width, 0f64, 0f64);
        let viewport_v = Vec3::new(0f64, -viewport_height, 0f64);

        let pixel_delta_u = viewport_u.clone() / image_width as f64;
        let pixel_delta_v = viewport_v.clone() / image_height as f64;

        let viewport_upper_left = center.clone()
            - Vec3::new(0f64, 0f64, focal_length)
            - (viewport_u.clone() / 2f64)
            - (viewport_v.clone() / 2f64);
        let pixel00_loc =
            viewport_upper_left.clone() + (pixel_delta_u.clone() + pixel_delta_v.clone()) * 0.5;

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
    pub fn render(&self, world: &dyn Hittable, mut writer: &mut BufWriter<File>) -> io::Result<()> {
        writer.write_all(
            format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes(),
        )?;
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc.clone()
                    + (self.pixel_delta_u.clone() * i as f64)
                    + (self.pixel_delta_v.clone() * j as f64);
                let ray_direction = pixel_center.clone() - self.center.clone();
                let r = Ray::new(self.center, ray_direction);

                let pixel_color = self.ray_color(&r, world);
                write_color(&mut writer, pixel_color)?;
            }
        }
        Ok(())
    }

    fn ray_color(&self, r: &Ray, world: &dyn Hittable) -> Vec3 {
        if let Some(rec) = world.hit(r, Interval::new(0f64, f64::MAX)) {
            return (rec.normal + Vec3::new(1f64, 1f64, 1f64)) * 0.5;
        }

        let unit_direction = r.direction.unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
    }
}
