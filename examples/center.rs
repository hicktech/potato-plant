use adafruit_motorkit::{dc::DcMotor, init_pwm, Motor};
use std::env::args;
use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut pwm = init_pwm(None)?;
    let mut dc_motor = DcMotor::try_new(&mut pwm, Motor::Motor1)?;
    dc_motor.set_throttle(&mut pwm, 1.0)?;
    thread::sleep(Duration::from_secs(2));
    dc_motor.set_throttle(&mut pwm, 0.0)?;
    dc_motor.set_throttle(&mut pwm, -1.0)?;
    thread::sleep(Duration::from_secs(2));
    dc_motor.set_throttle(&mut pwm, 0.0)?;
    dc_motor.set_throttle(&mut pwm, 0.5)?;
    thread::sleep(Duration::from_millis(1750));
    dc_motor.set_throttle(&mut pwm, 0.0)?;
    dc_motor.stop(&mut pwm)?;
    Ok(())
}
