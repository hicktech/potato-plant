use adafruit_motorkit::{dc::DcMotor, init_pwm, Motor};
use clap::Parser;
use std::error::Error;
use std::ops::Neg;
use std::thread;
use std::time::Duration;

#[derive(Parser)]
struct Opts {
    #[clap(short, default_value = "50")]
    time: u64,

    #[clap(long)]
    neg: bool,

    throttle: f32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();

    println!("time with {}", opts.time);
    println!("throttle with {}", opts.throttle);

    let mut pwm = init_pwm(None)?;
    let mut dc_motor = DcMotor::try_new(&mut pwm, Motor::Motor1)?;

    if opts.neg {
        dc_motor.set_throttle(&mut pwm, opts.throttle.neg())?;
    } else {
        dc_motor.set_throttle(&mut pwm, opts.throttle)?;
    }

    thread::sleep(Duration::from_millis(opts.time));
    dc_motor.set_throttle(&mut pwm, 0.0)?;
    dc_motor.stop(&mut pwm)?;
    Ok(())
}
