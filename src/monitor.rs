use crate::io::{Cmd, Event, IO};
use embedded_hal::digital::OutputPin;
use std::thread;

#[derive(Default)]
pub struct Monitor {
    pub ground_speed_mph: f32,
    pub feet_planted: f32,
    pub auto_prime: [bool; 2],
    pub priming: [bool; 2],
    pub planter_raised: bool,
    pub io: IO,
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
            Event::GroundSpeed(v) => self.ground_speed_mph = v,
        }
    }
}
