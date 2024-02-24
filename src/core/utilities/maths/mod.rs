pub fn abs(value: f32) -> f32 {
    if value < 0.0 {
        -value
    } else {
        value
    }
}

pub fn lerp(from: f32, to: f32, delta: f32) -> f32 {
    from * (1.0 - delta) + to * delta
}

pub fn sign(value: f32) -> f32 {
    if value < 0.0 {
        return -1.0;
    }
    if value > 0.0 {
        return 1.0;
    }
    0.0
}

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    value
}

pub fn move_towards(from: f32, to: f32, delta: f32) -> f32 {
    if abs(to - from) <= delta {
        return to;
    }
    from + (sign(to - from)) * delta
}

pub fn compute_deceleration(value: f32, delta: f32, deceleration: f32) -> f32 {
    move_towards(value, 0.0, deceleration * delta)
}

pub fn compute_acceleration(value: f32, delta: f32, max_speed: f32, acceleration: f32) -> f32 {
    move_towards(value, max_speed, acceleration * delta)
}
