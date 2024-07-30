use crate::vec3::Vec3;

pub(crate) struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub(crate) fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_at() {
        let l = Vec3::new(1.0, -1.0, 0.0);
        let r = Vec3::new(5.0, 2.5, -4.0);

        let r = Ray::new(l, r);
        let result = r.at(2.);

        assert_eq!(result, Vec3::new(11., 4., -8.));
    }
}
