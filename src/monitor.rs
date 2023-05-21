use crate::io::{Cmd, Event, IO};
use embedded_hal::digital::OutputPin;
use std::thread;

#[derive(Default)]
pub struct Monitor {
    pub io: IO,

    pub ground_speed_mph: f32,
    pub seed_wheel_speed_rpm: f32,
    pub planter_raised: bool,
    pub auto_prime: [bool; 2],
    pub priming: [bool; 2],

    pub feet_planted: f32,
}

impl Monitor {
    pub fn enable_seed_belt(&self, id: usize, en: bool) {
        self.io.tx.send(Cmd::SeedBeltControl(id, en));
    }

    pub fn halt(&self) {
        self.io.tx.send(Cmd::FlowHold);
    }

    pub fn handle_event(&mut self, e: Event) {
        match e {
            Event::SeedWheelTick => {}
            Event::PlanterRaised => self.planter_raised = true,
            Event::PlanterLowered => self.planter_raised = false,
            Event::GroundSpeed(mph) => self.ground_speed_mph = mph,
            Event::HopperEmpty(n) => self.priming[n] = true,
            Event::HopperFull(n) => self.priming[n] = false,
            Event::SeedWheelSpeed(rpm) => self.seed_wheel_speed_rpm = rpm,
        }
    }
}
