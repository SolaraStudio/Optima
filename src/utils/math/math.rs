pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn clamp_i32(value: i32, min: i32, max: i32) -> i32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn lerp_clamped(a: f32, b: f32, t: f32) -> f32 {
    lerp(a, b, clamp(t, 0.0, 1.0))
}

pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub fn map_range(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    to_min + (value - from_min) * (to_max - to_min) / (from_max - from_min)
}

pub fn map_range_clamped(
    value: f32,
    from_min: f32,
    from_max: f32,
    to_min: f32,
    to_max: f32,
) -> f32 {
    let t = clamp((value - from_min) / (from_max - from_min), 0.0, 1.0);
    to_min + t * (to_max - to_min)
}

pub fn lerp_angle(a: f32, b: f32, t: f32) -> f32 {
    let diff = b - a;
    let shortest =
        diff - 2.0 * std::f32::consts::PI * (diff / (2.0 * std::f32::consts::PI)).round();
    a + shortest * t
}

pub fn wrap_angle(angle: f32) -> f32 {
    ((angle % (2.0 * std::f32::consts::PI)) + 2.0 * std::f32::consts::PI)
        % (2.0 * std::f32::consts::PI)
}

pub fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

pub fn lerp_point(x1: f32, y1: f32, x2: f32, y2: f32, t: f32) -> (f32, f32) {
    (lerp(x1, x2, t), lerp(y1, y2, t))
}

pub fn angle_between(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (y2 - y1).atan2(x2 - x1)
}

pub fn random_f32() -> f32 {
    use rand::distributions::{Distribution, Standard};
    Standard.sample(&mut rand::thread_rng())
}

pub fn random_f32_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random_f32()
}

pub fn random_i32_range(min: i32, max: i32) -> i32 {
    use rand::Rng;
    rand::thread_rng().gen_range(min..=max)
}
