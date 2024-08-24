use std::rc::Rc;

use crate::interval::Interval;
use crate::material::Scatterable;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct HitRecord {
    pub(crate) p: Vec3,
    pub(crate) normal: Vec3,
    pub(crate) t: f64,
    pub(crate) front_face: bool,
    pub(crate) mat: Rc<dyn Scatterable>,
}

pub trait Hittable {
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
        let mut closest_so_far = ray_t.max;
        let mut temp_rec = None;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }
}
