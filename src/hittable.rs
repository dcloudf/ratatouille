use crate::ray::Ray;
use crate::vec3::Vec3;

pub(crate) struct HitRecord {
    pub(crate) p: Vec3,
    pub(crate) normal: Vec3,
    pub(crate) t: f64,
    pub(crate) front_face: bool,
}

impl HitRecord {
    pub(crate) fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0f64;
        self.normal = match self.front_face {
            true => *outward_normal,
            false => -(*outward_normal),
        }
    }
}

pub(crate) trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}
