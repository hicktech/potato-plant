#[derive(Default)]
pub struct Monitor {
    pub ground_speed_mph: f32,
    pub feet_planted: f32,
    pub auto_prime: [bool; 2],
    pub priming: [bool; 2],
}
