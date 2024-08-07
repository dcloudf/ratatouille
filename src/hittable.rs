use std::rc::Rc;

use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Default, Clone, Copy)]
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
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

pub(crate) struct HittableList {
    pub(crate) objects: Vec<Rc<dyn Hittable>>,
}

impl Default for HittableList {
    fn default() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
}

impl HittableList {
    pub(crate) fn new(object: Rc<dyn Hittable>) -> Self {
        HittableList {
            objects: vec![object],
        }
    }

    pub(crate) fn clear(&mut self) {
        self.objects.clear()
    }

    pub(crate) fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        let mut temp_rec = HitRecord::default();

        for object in &self.objects {
            if let Some(rec) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                hit_anything = true;
                closest_so_far = rec.t;
                temp_rec = rec;
            }
        }

        match hit_anything {
            true => Some(temp_rec),
            false => None,
        }
    }
}
