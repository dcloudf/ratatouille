pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn infinity() -> Self {
        Interval {
            min: f64::MIN,
            max: f64::MAX,
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, test: f64) -> bool {
        self.min <= test && test <= self.max
    }

    pub fn surrounds(&self, test: f64) -> bool {
        self.min < test && test < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        } else if self.max < x {
            return self.max;
        } else {
            return x;
        }
    }
}
