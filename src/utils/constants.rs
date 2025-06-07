use super::interval::Interval;

pub const PI: f32 = 3.141_592_7;
pub const INFINITY: f32 = f32::MAX;
pub const MAX_OBJECTS_ONSCREEN: u32 = 100;
pub const EPSILON: f32 = 1e-8;
pub const UNIVERSE: Interval = Interval {
    min: -INFINITY,
    max: INFINITY,
};
pub const EMPTY: Interval = Interval {
    min: INFINITY,
    max: -INFINITY,
};
pub const INTENSITY: Interval = Interval {
    min: 0.0,
    max: 0.999,
};
