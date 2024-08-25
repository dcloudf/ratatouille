use rand::prelude::*;

use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};

pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Vec3,
    pub is_scattered: bool,
}

pub trait Scatterable {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> ScatterResult;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> ScatterResult {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        ScatterResult {
            scattered,
            attenuation,
            is_scattered: true,
        }
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        let fuzz = match fuzz < 1. {
            true => 1.,
            false => fuzz,
        };
        Self { albedo, fuzz }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> ScatterResult {
        let reflected = r_in.direction.reflect(rec.normal).unit_vector()
            + (Vec3::random_unit_vector() * self.fuzz);
        ScatterResult {
            scattered: Ray::new(rec.p, reflected),
            attenuation: self.albedo,
            is_scattered: true,
        }
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = ((1. - refraction_index) / (1. + refraction_index)).powi(2);
    r0 + (1. - r0) * (1. - cosine).powi(5)
}

impl Scatterable for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> ScatterResult {
        let mut rng = thread_rng();
        let attenuation = Vec3::new(1., 1., 1.);
        let ri = match rec.front_face {
            true => 1. / self.refraction_index,
            false => self.refraction_index,
        };
        let unit_direction = r_in.direction.unit_vector();
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let direction = match (ri * sin_theta > 1.) || reflectance(cos_theta, ri) > rng.gen::<f64>()
        {
            true => unit_direction.reflect(rec.normal),
            false => unit_direction.refract(rec.normal, ri),
        };
        ScatterResult {
            scattered: Ray::new(rec.p, direction),
            attenuation,
            is_scattered: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflectance() {
        assert_eq!(reflectance(0.0, 1.5), 1.0)
    }
}
