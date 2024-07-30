pub(crate) use color::write_color;
use ray::Ray;
use std::fs::File;
use std::io::{self, BufWriter, Write};
pub(crate) use vec3::Vec3;

pub mod color;
pub mod hittable;
pub mod ray;
pub mod sphere;
pub mod vec3;

fn hit_sphere(center: &Vec3, raduis: f64, r: &Ray) -> f64 {
    let oc = (*center).clone() - r.origin;
    let a = r.direction.len_squared();
    let h = r.direction.dot(&oc);
    let c = oc.len_squared() - raduis * raduis;
    let discriminant = h * h - a * c;
    match discriminant {
        0f64.. => return (h - discriminant.sqrt()) / a,
        _ => -1f64,
    }
}

fn ray_color(r: &Ray) -> Vec3 {
    let t = hit_sphere(&Vec3::new(0f64, 0f64, -1f64), 0.5, r);
    if t > 0.0 {
        let N = r.at(t) - Vec3::new(0f64, 0f64, -1f64);
        return Vec3::new(N.x() + 1f64, N.y() + 1f64, N.z() + 1f64) * 0.5;
    }

    let unit_direction = r.direction.unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
}

fn main() -> io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = match image_width as f64 / aspect_ratio {
        ..=1f64 => 1,
        n => n as i64,
    };

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vec3::new(0f64, 0f64, 0f64);

    let viewport_u = Vec3::new(viewport_width, 0f64, 0f64);
    let viewport_v = Vec3::new(0f64, -viewport_height, 0f64);

    let pixel_delta_u = viewport_u.clone() / image_width as f64;
    let pixel_delta_v = viewport_v.clone() / image_height as f64;

    let viewport_upper_left = camera_center.clone()
        - Vec3::new(0f64, 0f64, focal_length)
        - (viewport_u.clone() / 2f64)
        - (viewport_v.clone() / 2f64);
    let pixel00_loc =
        viewport_upper_left.clone() + (pixel_delta_u.clone() + pixel_delta_v.clone()) * 0.5;

    let file = File::create("image.ppm")?;
    let mut writer = BufWriter::new(file);
    writer.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())?;

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel00_loc.clone()
                + (pixel_delta_u.clone() * i as f64)
                + (pixel_delta_v.clone() * j as f64);
            let ray_direction = pixel_center.clone() - camera_center.clone();
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r);
            write_color(&mut writer, pixel_color)?;
        }
    }
    Ok(())
}
