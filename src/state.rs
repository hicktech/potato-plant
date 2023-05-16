use crate::io::{Cmd, IO};
use embedded_hal::digital::OutputPin;

#[derive(Default)]
pub struct Monitor {
    pub ground_speed_mph: f32,
    pub feet_planted: f32,
    pub auto_prime: [bool; 2],
    pub priming: [bool; 2],
    pub io: IO,
}

impl Monitor {
    pub fn enable_seed_belt(&self, id: usize, en: bool) {
        self.io.tx.send(Cmd::SeedBeltControl(id, en));
    }

    pub fn halt(&self) {
        self.io.tx.send(Cmd::FlowHold);
    }
}
