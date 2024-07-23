use crate::vec3::Vec3;

pub(crate) struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    pub(crate) fn new(origin: &Vec3, direction: &Vec3) -> Self {
        Ray {
            orig: origin.clone(),
            dir: direction.clone(),
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.orig.clone()
    }

    pub fn direction(&self) -> Vec3 {
        self.dir.clone()
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig.clone() + self.dir.clone() * t
    }
}
