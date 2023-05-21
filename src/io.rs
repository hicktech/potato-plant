use crate::io::Event::{HopperEmpty, HopperFull, PlanterLowered, PlanterRaised};
use adafruit_motorkit::dc::DcMotor;
use adafruit_motorkit::{init_pwm, Motor};
use crossbeam_channel::{Receiver, Sender};
use embedded_hal::digital::OutputPin;
use rppal::gpio::{Gpio, Trigger};
use std::error::Error;
use std::thread;

#[derive(Default)]
pub enum LiftSensor {
    #[default]
    Software,
    Hardware {
        pin: u8,
    },
}

pub struct IoCfg {
    pub seed_belt_pins: [u8; 2],
    pub seed_wheel_speed_pin: u8,
    pub lift_sensor: LiftSensor,
}

impl Default for IoCfg {
    fn default() -> Self {
        IoCfg {
            seed_belt_pins: [4, 5],
            seed_wheel_speed_pin: 18,
            lift_sensor: Default::default(),
        }
    }
}

impl Default for IO {
    fn default() -> Self {
        IO::fake(IoCfg::default()).expect("==gpio init error==")
    }
}

#[derive(Debug, Clone)]
pub enum Cmd {
    SeedBeltControl(usize, bool),
    FlowThrottle(f32),
    FlowHold,
    RaisePlanter,
    LowerPlanter,
}

#[derive(Debug, Clone)]
pub enum Event {
    SeedWheelTick,
    PlanterRaised,
    PlanterLowered,
    GroundSpeed(f32),
    SeedWheelSpeed(f32),
    HopperEmpty(usize),
    HopperFull(usize),
}

pub struct IO {
    pub tx: Sender<Cmd>,
    pub rx: Receiver<Event>,
}

impl IO {
    pub fn new(cfg: IoCfg) -> Result<Self, Box<dyn Error>> {
        let mut belt = cfg.seed_belt_pins.map(|p| {
            Gpio::new()
                .expect("gpio1")
                .get(cfg.seed_belt_pins[p as usize])
                .expect("gpio2")
                .into_output_low()
        });

        let mut pwm = init_pwm(None)?;
        let mut dc_motor = DcMotor::try_new(&mut pwm, Motor::Motor1)?;

        let (tx, crx) = crossbeam_channel::unbounded();
        let (etx, rx) = crossbeam_channel::unbounded();

        let mut speed = Gpio::new()?.get(cfg.seed_wheel_speed_pin)?.into_input();
        speed
            .set_async_interrupt(Trigger::RisingEdge, move |x| {
                etx.send(Event::SeedWheelTick);
            })
            .expect("failed to add listener to seed wheel");

        thread::spawn(move || {
            for cmd in crx {
                match cmd {
                    Cmd::SeedBeltControl(id, en) => {
                        belt[id].set_state(en.into());
                    }
                    Cmd::FlowThrottle(rate) => {
                        dc_motor.set_throttle(&mut pwm, rate).unwrap();
                    }
                    Cmd::FlowHold => {
                        dc_motor.stop(&mut pwm);
                    }
                    _ => {}
                }
            }
        });

        Ok(IO { tx, rx })
    }

    pub fn fake(cfg: IoCfg) -> Result<Self, Box<dyn Error>> {
        let (tx, crx) = crossbeam_channel::unbounded();
        let (etx, rx) = crossbeam_channel::unbounded();

        // todo;; timer to simulate wheel speed

        thread::spawn(move || {
            for cmd in crx {
                match cmd {
                    Cmd::SeedBeltControl(id, en) => {
                        println!("Belt {id} {}", if en { "enabled" } else { "disabled" });
                        etx.send(if en { HopperEmpty(id) } else { HopperFull(id) });
                    }
                    Cmd::FlowThrottle(rate) => {
                        println!("Flow rate set to {rate}")
                    }
                    Cmd::FlowHold => {
                        println!("Flow rate stopped")
                    }
                    Cmd::RaisePlanter => {
                        etx.send(PlanterRaised);
                        println!("Raise planter")
                    }
                    Cmd::LowerPlanter => {
                        etx.send(PlanterLowered);
                        println!("Lower planter")
                    }
                }
            }
        });

        Ok(IO { tx, rx })
    }
}
