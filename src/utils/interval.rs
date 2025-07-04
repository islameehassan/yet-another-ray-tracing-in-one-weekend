#[derive(Copy, Clone, Debug)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(mi: f32, ma: f32) -> Self {
        Interval { min: mi, max: ma }
    }

    pub fn from(interval: &mut Self) -> Self {
        Interval {
            min: interval.min,
            max: interval.max,
        }
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, value: f32) -> bool {
        value >= self.min && value <= self.max
    }

    pub fn surrounds(&self, value: f32) -> bool {
        value < self.min || value > self.max
    }

    pub fn clamp(&self, value: f32) -> f32 {
        if value < self.min {
            return self.min;
        } else if value > self.max {
            return self.max;
        }
        value
    }
}
