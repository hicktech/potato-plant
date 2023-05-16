pub fn row_feet_to_acres(ft: f32) -> f32 {
    ft / 14520.0
}

pub fn mph_to_fps(mph: f32) -> f32 {
    mph * 1.467
}

pub fn fps_to_sps(fps: f32, in_between: f32) -> f32 {
    fps * 12.0 / in_between
}
