use super::vec3::Vec3;
use crate::utils::constants::INTENSITY;
pub type Color3 = Vec3;

#[inline]
fn linear_to_gamma(linear_cmp: f32) -> f32 {
    linear_cmp.sqrt()
}

pub fn write_color(file: &mut dyn std::io::Write, color: Color3) {
    let r = linear_to_gamma(color.x);
    let g = linear_to_gamma(color.y);
    let b = linear_to_gamma(color.z);

    let rbyte = (256.0 * INTENSITY.clamp(r)) as u8;
    let gbyte = (256.0 * INTENSITY.clamp(g)) as u8;
    let bbyte = (256.0 * INTENSITY.clamp(b)) as u8;
    writeln!(file, "{} {} {}", rbyte, gbyte, bbyte).unwrap();
}
