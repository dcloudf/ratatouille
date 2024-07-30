use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub(crate) struct Sphere {
    center: Vec3,
    raduis: f64,
}

impl Sphere {
    fn new(center: Vec3, raduis: f64) -> Self {
        Sphere {
            center,
            raduis: raduis.max(0f64),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let oc = self.center.clone() - r.origin;
        let a = r.direction.len_squared();
        let h = r.direction.dot(&oc);
        let c = oc.len_squared() - self.raduis * self.raduis;
        let discriminant = h * h - a * c;
        if discriminant < 0f64 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            let root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        };

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.raduis;
        rec.set_face_normal(r, &outward_normal);

        true
    }
}
