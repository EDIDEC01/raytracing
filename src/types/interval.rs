use crate::INFINITY;

#[derive(Clone, Default, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    #[inline]
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
    }
}

#[allow(dead_code)] const EMPTY: Interval = Interval::new(INFINITY, -INFINITY);
#[allow(dead_code)] const UNIVERSE: Interval = Interval::new(-INFINITY, INFINITY);
pub const POSITIVE: Interval = Interval::new(0.001, INFINITY);