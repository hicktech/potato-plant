use adafruit_motorkit::dc::DcMotor;
use adafruit_motorkit::{init_pwm, Motor};
use std::error::Error;
use std::thread;
use std::time::Duration;

use clap::Parser;
use popl::gps;
use rppal::gpio::{Gpio, Level, Trigger};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::{select, time};

#[derive(Parser)]
struct Opts {
    #[clap(default_value = "10")]
    spacing: usize,

    #[clap(long)]
    speed: Option<f32>,
}

struct EncoderTick;
enum HopperState {
    Empty,
    Full,
}

enum PlanterLiftState {
    Raised,
    Lowered,
}

#[derive(Debug)]
enum Message {
    GroundSpeed(f32),
    TickRate(f32),
    HopperFull(usize),
    HopperEmpty(usize),
    PlanterRaised,
    PlanterLowered,
}

// #4
const CLOCK_PIN: u8 = 26;
// #5
const DATA_PIN: u8 = 27;

// #6
const HOPPER_LIMIT_0: u8 = 24;
// #7
const HOPPER_LIMIT_1: u8 = 25;

// #8
const HOPPER_RELAY_0: u8 = 5;
// #9
const HOPPER_RELAY_1: u8 = 6;

// #10
const PLANTER_LIFT_PIN: u8 = 4;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();

    // main messaging channel; analogous to the iced update function or subscription channel
    let (msg_tx, mut msg_rx) = mpsc::unbounded_channel();

    // ground speed
    let (speed_tx, mut speed_rx) = mpsc::channel(1);
    let mut ground_speed = if let Some(set_speed) = opts.speed {
        speed_tx.send(gps::GroundSpeed::Gps(set_speed));
        set_speed
    } else {
        tokio::spawn(async {
            gps::read_speed(speed_tx, "/dev/ttyACM0")
                .await
                .expect("gps read")
        });
        0.0f32
    };

    // flow control
    let mut pwm = init_pwm(None)?;
    let mut dc_motor = DcMotor::try_new(&mut pwm, Motor::Motor1)?;

    // hopper specific channels, these are aggregated into main message channel
    let (hopper0_tx, mut hopper0_rx) = mpsc::channel(1);
    let (hopper1_tx, mut hopper1_rx) = mpsc::channel(1);

    // limit switch per hopper
    let mut limit_pin_hopper0 = Gpio::new()?.get(HOPPER_LIMIT_0)?.into_input_pulldown();
    let mut limit_pin_hopper1 = Gpio::new()?.get(HOPPER_LIMIT_1)?.into_input_pulldown();

    limit_pin_hopper0.set_async_interrupt(Trigger::Both, move |l| {
        match l {
            Level::Low => hopper0_tx.blocking_send(HopperState::Empty),
            Level::High => hopper0_tx.blocking_send(HopperState::Full),
        };
    })?;

    limit_pin_hopper1.set_async_interrupt(Trigger::Both, move |l| {
        match l {
            Level::Low => hopper1_tx.blocking_send(HopperState::Empty),
            Level::High => hopper1_tx.blocking_send(HopperState::Full),
        };
    })?;

    // hopper feed belts relay control pins
    let mut hopper_relay_pins = [
        Gpio::new()?.get(HOPPER_RELAY_0)?.into_output(),
        Gpio::new()?.get(HOPPER_RELAY_1)?.into_output(),
    ];

    // planter channel reports planter lift state to main message channel
    let (planter_lift_tx, mut planter_lift_rx) = mpsc::channel(1);
    let mut limit_planter_lift = Gpio::new()?.get(PLANTER_LIFT_PIN)?.into_input_pulldown();
    limit_planter_lift.set_async_interrupt(Trigger::Both, move |l| {
        match l {
            Level::Low => planter_lift_tx.blocking_send(PlanterLiftState::Lowered),
            Level::High => planter_lift_tx.blocking_send(PlanterLiftState::Raised),
        };
    })?;

    // encoder channel reports encoder ticks to main message channel
    let (tick_tx, mut tick_rx) = mpsc::channel(1);
    thread::spawn(move || {
        read_encoder(tick_tx);
    });

    // aggregate various inputs into messages
    tokio::task::spawn(async move {
        // timeout used to aggrated ticks into tick per second measurement
        let mut interval = time::interval(Duration::from_secs(1));

        // ticks in the current second
        let mut tps = 0i32;

        //
        loop {
            select! {
                Some(h0state) = hopper0_rx.recv() => {
                    match h0state {
                        HopperState::Full => {msg_tx.send(Message::HopperFull(0)) ;}
                        HopperState::Empty => {msg_tx.send(Message::HopperEmpty(0)) ;}
                    }
                },
                Some(h1state) = hopper1_rx.recv() => {
                    match h1state {
                        HopperState::Full => {msg_tx.send(Message::HopperFull(1)) ;}
                        HopperState::Empty => {msg_tx.send(Message::HopperEmpty(1)) ;}
                    }
                },
                Some(e) = tick_rx.recv() => {tps += 1;},
                Some(lift_state) = planter_lift_rx.recv() => {
                    match lift_state {
                        PlanterLiftState::Raised => { msg_tx.send(Message::PlanterRaised); }
                        PlanterLiftState::Lowered => { msg_tx.send(Message::PlanterLowered); }
                    }
                },
                Some(fix) = speed_rx.recv() => {
                    match fix {
                        gps::GroundSpeed::Gps(speed) => {
                            msg_tx.send(Message::GroundSpeed(speed));
                        },
                        _ => {}
                    }
                },
                _ = interval.tick() => {
                    msg_tx.send(Message::TickRate(tps as f32));
                    tps = 0;
                }
            }
        }
    });

    // the decision maker
    use popl::util::*;
    use Message::*;

    let mut timeout = time::interval(Duration::from_secs(1));
    let seed_spacing = 10.0;

    loop {
        let fps = mph_to_fps(ground_speed);
        let target_sps = fps_to_sps(fps, seed_spacing);
        //let target_tps = 10.0;
        let target_tps = ticks_per_pick() as f32 * target_sps;

        select! {
            Some(msg) = msg_rx.recv() => {
                match msg {
                    GroundSpeed(speed) => {
                        println!("Ground Speed: {speed}");
                        println!("target sps {target_sps} for speed of {speed}");
                        ground_speed = speed;
                    },
                    HopperFull(i) => hopper_relay_pins[i].write(Level::High),
                    HopperEmpty(i) => hopper_relay_pins[i].write(Level::Low),
                    TickRate(tps) => {
                        // automatically adjust the flow control
                        // todo;; perhaps automatic is not the best way to begin
                        match target_tps {
                            target if tps < target => {
                                println!("increase flow");
                                // increase flow
                                dc_motor.set_throttle(&mut pwm, 0.5).expect("throttle");
                                thread::sleep(Duration::from_millis(500));
                                dc_motor.set_throttle(&mut pwm, 0.0).expect("throttle2");
                            }
                            target if tps > target => {
                                println!("reduce flow");
                                // reduce flow
                                dc_motor.set_throttle(&mut pwm, -0.5).expect("throttle");
                                thread::sleep(Duration::from_millis(500));
                                dc_motor.set_throttle(&mut pwm, 0.0).expect("throttle2");
                            }
                            _ => {
                                // hold position
                                dc_motor.set_throttle(&mut pwm, 0.0).expect("throttle");
                                dc_motor.stop(&mut pwm).expect("stop");
                            }
                        }
                    }
                    PlanterRaised => println!("planter raised"),
                    PlanterLowered => println!("planter lowered"),
                }
            }
            _ = timeout.tick() => {
                continue;
            }
        }
    }
}

fn read_encoder(mut tx: Sender<EncoderTick>) -> Result<(), Box<dyn Error>> {
    use Level::*;

    let clock = Gpio::new()?.get(CLOCK_PIN)?.into_input_pullup();
    let data = Gpio::new()?.get(DATA_PIN)?.into_input_pullup();

    let mut state: u16 = 0;
    let mut encoder_idx: i64 = 0;

    while !tx.is_closed() {
        let c = clock.read() as u16;
        let d = data.read() as Level;

        state = (&state << 1) | c | 0xe000;
        if state == 0xf000 {
            match d {
                High => encoder_idx += 1,
                Low => encoder_idx -= 1,
            }

            state = 0;
            tx.blocking_send(EncoderTick);
            println!("idx {}", encoder_idx);
        }
    }

    Ok(())
}
