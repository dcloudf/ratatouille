use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Scatterable;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub(crate) struct Sphere {
    center: Vec3,
    raduis: f64,
    mat: Rc<dyn Scatterable>,
}

impl Sphere {
    pub(crate) fn new(center: Vec3, raduis: f64, mat: Rc<dyn Scatterable>) -> Self {
        Sphere {
            center,
            raduis: raduis.max(0f64),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center.clone() - r.origin;
        let a = r.direction.len_squared();
        let h = r.direction.dot(&oc);
        let c = oc.len_squared() - self.raduis * self.raduis;
        let discriminant = h * h - a * c;
        if discriminant < 0f64 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            let root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        };

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.raduis;
        let front_face = r.direction.dot(&outward_normal) < 0f64;
        let normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };

        Some(HitRecord {
            p,
            normal,
            t: root,
            front_face,
            mat: self.mat.clone(),
        })
    }
}
