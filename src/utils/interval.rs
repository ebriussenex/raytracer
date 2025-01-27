pub struct Interval(f64, f64);

pub const EMPTY: Interval = Interval(f64::INFINITY, f64::NEG_INFINITY);
pub const UNIVERSE: Interval = Interval(f64::NEG_INFINITY, f64::INFINITY);

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval(min, max)
    }

    pub fn min(&self) -> f64 {
        self.0
    }

    pub fn max(&self) -> f64 {
        self.1
    }

    pub fn size(&self) -> f64 {
        self.1 - self.0
    }

    pub fn contains(&self, val: f64) -> bool {
        self.0 <= val && self.1 >= val
    }

    pub fn surrounds(&self, val: f64) -> bool {
        self.0 < val && self.1 > val
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval(f64::NEG_INFINITY, f64::INFINITY)
    }
}
