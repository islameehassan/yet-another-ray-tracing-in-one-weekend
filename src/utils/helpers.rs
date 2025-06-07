use super::constants::PI;
use fastrand;

pub fn degress_to_radians(degress: f32) -> f32 {
    degress * PI / 180.0
}

pub fn random_f32() -> f32 {
    fastrand::f32()
}

pub fn random_f32_with_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random_f32()
}
