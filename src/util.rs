// todo;; count the number of picks on the wheel
const PICKS_ON_WHEEL: usize = 12;

// todo;; confirm the number of ticks per rev
const REVOLUTION_TICKS: usize = 360;

pub fn row_feet_to_acres(ft: f32) -> f32 {
    ft / 14520.0
}

pub fn mph_to_fps(mph: f32) -> f32 {
    mph * 1.467
}

pub fn fps_to_sps(fps: f32, in_between: f32) -> f32 {
    fps * 12.0 / in_between
}

// since we do not index the wheel this is always an approximation
// representing how many seeds could fall in number of rev ticks
pub fn ticks_per_pick() -> usize {
    REVOLUTION_TICKS / PICKS_ON_WHEEL
}

pub fn seed_per_ticks(ticks: usize) -> usize {
    ticks / ticks_per_pick()
}
