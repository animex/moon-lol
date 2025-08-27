use std::f32::consts::SQRT_2;

use bevy::math::{Quat, Vec3};

const ONE_OVER_USHORT_MAX: f32 = 1.0 / 65535.0;
const ONE_OVER_SQRT_2: f32 = 1.0 / SQRT_2;

pub fn decompress_time(time: u16, duration: f32) -> f32 {
    time as f32 * ONE_OVER_USHORT_MAX * duration
}

pub fn decompress_vector3(value: &[u16; 3], min: &Vec3, max: &Vec3) -> Vec3 {
    let mut uncompressed = max - min;

    uncompressed.x *= value[0] as f32 * ONE_OVER_USHORT_MAX;
    uncompressed.y *= value[1] as f32 * ONE_OVER_USHORT_MAX;
    uncompressed.z *= value[2] as f32 * ONE_OVER_USHORT_MAX;

    uncompressed + min
}

pub fn decompress_quat(value: &[u16; 3]) -> Quat {
    let bits = (value[0] as u64) | ((value[1] as u64) << 16) | ((value[2] as u64) << 32);

    let max_index = (bits >> 45) & 0x03;
    let v_a = (bits >> 30) & 0x7FFF;
    let v_b = (bits >> 15) & 0x7FFF;
    let v_c = bits & 0x7FFF;

    let a = (v_a as f32 / 32767.0) * SQRT_2 - ONE_OVER_SQRT_2;
    let b = (v_b as f32 / 32767.0) * SQRT_2 - ONE_OVER_SQRT_2;
    let c = (v_c as f32 / 32767.0) * SQRT_2 - ONE_OVER_SQRT_2;

    let sub = 1.0 - (a * a + b * b + c * c);
    let d = f32::sqrt(f32::max(0.0, sub));

    match max_index {
        0 => Quat::from_xyzw(d, a, b, c),
        1 => Quat::from_xyzw(a, d, b, c),
        2 => Quat::from_xyzw(a, b, d, c),
        _ => Quat::from_xyzw(a, b, c, d),
    }
}
