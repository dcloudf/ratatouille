use crate::vec3::Vec3;

pub(crate) struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    fn new(origin: &Vec3, direction: &Vec3) -> Self {
        Ray {
            orig: origin.clone(),
            dir: direction.clone(),
        }
    }

    fn origin(&self) -> Vec3 {
        self.orig.clone()
    }

    fn direction(&self) -> Vec3 {
        self.dir.clone()
    }

    fn at(&self, t: f64) -> Vec3 {
        self.orig.clone() + self.dir.clone() * t
    }
}
