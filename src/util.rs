// rate of encoder ticks per seconc
pub type TickRate = usize;
// rate of picks per second
pub type PickRate = f32;
// rate of seed per second
pub type SeedRate = PickRate;
pub type Speed = f32;

// 12 picks per wheel, 2 wheels per row
const REVOLUTION_PICKS: f32 = 24.0;

// 100 tick encoder steps per seed wheel revolution
const REVOLUTION_TICKS: f32 = 340.0;

pub fn row_feet_to_acres(ft: f32) -> f32 {
    ft / 14520.0
}

pub fn mph_to_fps(mph: Speed) -> f32 {
    mph * 1.467
}

pub fn fps_to_sps(fps: Speed, in_between: f32) -> SeedRate {
    fps * 12.0 / in_between
}

// since we do not index the wheel this is always an approximation
// representing how many seeds could fall in number of rev ticks
pub fn ticks_per_pick() -> f32 {
    REVOLUTION_TICKS / REVOLUTION_PICKS
}

pub fn seed_per_ticks(ticks: TickRate) -> SeedRate {
    ticks as f32 / ticks_per_pick() as f32
}

pub fn rpm_to_seed_per_second(rpm: f32) -> f32 {
    rpm * 60.0 / REVOLUTION_PICKS as f32
}

// 10 seeds per second
// spaced 1 ft
//
pub fn sps_to_fps(sps: SeedRate, in_between: f32) -> Speed {
    sps * (in_between / 12.0)
}

pub fn sps_to_mph(sps: SeedRate, in_between: f32) -> Speed {
    sps_to_fps(sps, in_between) / 1.467
}

pub fn sps_to_tickrate(sps: SeedRate) -> TickRate {
    (sps * ticks_per_pick()) as TickRate
}

pub fn sps_from_tickrate(tickrate: TickRate) -> SeedRate {
    tickrate as f32 / ticks_per_pick() as f32
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

        //assert_eq!(seed_per_ticks(340), 24);

        assert_eq!(mph_to_fps(10.0), 14.67);

        assert!(sps_to_mph(10.0, 12.0) > sps_to_mph(10.0, 10.0));

        println!("{}", sps_to_mph(10.0, 8.0));
        println!("{}", sps_to_mph(10.0, 10.0));
        println!("{}", sps_to_mph(10.0, 12.0));
        println!("{}", sps_to_mph(10.0, 14.0));
    }
}
