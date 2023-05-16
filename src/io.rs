use adafruit_motorkit::dc::DcMotor;
use adafruit_motorkit::{init_pwm, Motor};
use crossbeam_channel::Sender;
use embedded_hal::digital::OutputPin;
use rppal::gpio::Gpio;
use std::error::Error;
use std::thread;

pub struct IoCfg {
    pub seed_belt_pins: [u8; 2],
}

impl Default for IoCfg {
    fn default() -> Self {
        IoCfg {
            seed_belt_pins: [4, 5],
        }
    }
}

impl Default for IO {
    fn default() -> Self {
        IO::fake(IoCfg::default()).expect("==gpio init error==")
    }
}

pub enum Cmd {
    SeedBeltControl(usize, bool),
    FlowThrottle(f32),
    FlowHold,
}

pub struct IO {
    pub tx: Sender<Cmd>,
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

        let (tx, rx) = crossbeam_channel::unbounded();
        thread::spawn(move || {
            for cmd in rx {
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
                }
            }
        });

        Ok(IO { tx })
    }

    pub fn fake(cfg: IoCfg) -> Result<Self, Box<dyn Error>> {
        let (tx, rx) = crossbeam_channel::unbounded();
        thread::spawn(move || {
            for cmd in rx {
                match cmd {
                    Cmd::SeedBeltControl(id, en) => {
                        println!("Belt {id} {}", if en { "enabled" } else { "disabled" })
                    }
                    Cmd::FlowThrottle(rate) => {
                        println!("Flow rate set to {rate}")
                    }
                    Cmd::FlowHold => {
                        println!("Flow rate stopped")
                    }
                }
            }
        });

        Ok(IO { tx })
    }
}
