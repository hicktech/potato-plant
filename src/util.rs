// 12 picks per wheel, 2 wheels per row
const REVOLUTION_PICKS: usize = 24;

// 100 tick encoder steps per seed wheel revolution
const REVOLUTION_TICKS: usize = 340;

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
    REVOLUTION_TICKS / REVOLUTION_PICKS
}

pub fn seed_per_ticks(ticks: usize) -> f32 {
    ticks as f32 / ticks_per_pick() as f32
}

pub fn rpm_to_seed_per_second(rpm: f32) -> f32 {
    rpm * 60.0 / REVOLUTION_PICKS as f32
}

// 10 seeds per second
// spaced 1 ft
//
pub fn sps_to_fps(sps: f32, in_between: f32) -> f32 {
    sps * (12.0 / in_between)
}

pub fn sps_to_mph(sps: f32, in_between: f32) -> f32 {
    sps_to_fps(sps, in_between) / 1.467
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let x = sps_to_fps(10.0, 12.0);
        assert_eq!(x, 10.0);

        let x = sps_to_mph(10.0, 12.0);
        assert_eq!(x, 6.8166327);

        assert_eq!(seed_per_ticks(340), 24);

        assert_eq!(mph_to_fps(10.0), 14.67)
    }
}
