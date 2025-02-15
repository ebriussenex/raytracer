#[derive(Clone, Copy, Debug)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

pub const EMPTY: Interval = Interval {
    min: f64::INFINITY,
    max: f64::NEG_INFINITY,
};

pub const UNIVERSE: Interval = Interval {
    max: f64::INFINITY,
    min: f64::NEG_INFINITY,
};

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn enclosing(lhs: &Interval, rhs: &Interval) -> Self {
        Interval {
            min: if lhs.min <= rhs.min { lhs.min } else { rhs.min },
            max: if lhs.max >= rhs.max { lhs.max } else { rhs.max },
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, val: f64) -> bool {
        self.min <= val && self.max >= val
    }

    pub fn surrounds(&self, val: f64) -> bool {
        self.min < val && self.max > val
    }

    pub fn expand(&self, delta: f64) -> Self {
        Interval::new(self.min - delta, self.max + delta)
    }
}

impl Default for Interval {
    fn default() -> Self {
        EMPTY
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max
    }
}
