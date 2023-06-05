use adafruit_motorkit::dc::DcMotor;
use adafruit_motorkit::{init_pwm, Motor};
use std::error::Error;
use std::ops::Neg;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use build_time::build_time_local;
use clap::Parser;
use crossbeam_channel::tick;
use popl::gps;
use rppal::gpio::{Gpio, Level, Trigger};
use rppal::pwm::Pwm;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::time::interval;
use tokio::{select, time};

#[derive(Parser)]
struct Opts {
    /// target seed spacing
    #[clap(long, default_value = "10.0")]
    spacing: f32,

    /// fixed speed
    #[clap(long)]
    speed: Option<f32>,

    /// debounce time for switches (millis)
    #[clap(long, default_value = "50")]
    debounce_time: u128,

    #[clap(long, default_value = "1.0")]
    throttle_rate: f32,

    #[clap(long, default_value = "50")]
    throttle_time: u64,

    #[clap(long)]
    disable_on_lift: bool,

    #[clap(long)]
    disable_speed: bool,

    /// timeout for event loop (millis)
    #[clap(long, default_value = "50")]
    event_loop_time: u64,

    #[clap(short, long)]
    quiet: bool,
}

struct EncoderTick;
struct EncoderTickRate(f32);

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
    let local_build_time = build_time_local!("%Y-%m-%dT%H:%M:%S%.f%:z");

    let opts: Opts = Opts::parse();

    let seed_spacing = opts.spacing;
    println!("build time: {local_build_time}");
    println!("seed spacing: {seed_spacing}");
    println!("fixed speed: {:?}", opts.speed);

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
    let mut limit_pin_hopper0 = Gpio::new()?.get(HOPPER_LIMIT_0)?.into_input_pullup();
    let mut limit_pin_hopper1 = Gpio::new()?.get(HOPPER_LIMIT_1)?.into_input_pullup();

    let mut hopper_check_interval = time::interval(Duration::from_secs(1));
    tokio::spawn(async move {
        loop {
            match limit_pin_hopper0.read() {
                Level::Low => hopper0_tx.send(HopperState::Empty).await,
                Level::High => hopper0_tx.send(HopperState::Full).await,
            };
            match limit_pin_hopper1.read() {
                Level::Low => hopper1_tx.send(HopperState::Empty).await,
                Level::High => hopper1_tx.send(HopperState::Full).await,
            };
            hopper_check_interval.tick().await;
        }
    });

    // hopper feed belts relay control pins
    let mut hopper_relay_pins = [
        Gpio::new()?.get(HOPPER_RELAY_0)?.into_output(),
        Gpio::new()?.get(HOPPER_RELAY_1)?.into_output(),
    ];

    // planter channel reports planter lift state to main message channel
    let (planter_lift_tx, mut planter_lift_rx) = mpsc::channel(1);
    let mut limit_planter_lift = Gpio::new()?.get(PLANTER_LIFT_PIN)?.into_input_pullup();

    let debounce_millis = opts.debounce_time;
    let mut limit_planter_lift_debounce = Instant::now();
    limit_planter_lift.set_async_interrupt(Trigger::Both, move |l| {
        let now = Instant::now();
        if now.duration_since(limit_planter_lift_debounce).as_millis() >= debounce_millis {
            limit_planter_lift_debounce = now;
            match l {
                Level::Low => planter_lift_tx.blocking_send(PlanterLiftState::Lowered),
                Level::High => planter_lift_tx.blocking_send(PlanterLiftState::Raised),
            };
        }
    })?;

    // encoder channel reports encoder ticks to main message channel
    let (tick_tx, mut tick_rx) = mpsc::channel(1);
    thread::spawn(move || {
        read_encoder(tick_tx);
    });

    //let (tickrate_tx, mut tickrate_rx) = mpsc::channel(1);
    let tickrate = Arc::new(AtomicU32::new(0));
    tokio::spawn({
        let tickrate = tickrate.clone();
        async move {
            // aggrated ticks into tick per second measurement
            let mut interval = time::interval(Duration::from_secs(1));
            let mut current_ticks = 0;

            loop {
                select! {
                    Some(e) = tick_rx.recv() => { current_ticks += 1; },
                     _ = interval.tick() => {
                         //tickrate_tx.send(EncoderTickRate(tps as f32));
                         let last_rate = tickrate.load(Ordering::Relaxed);
                         if last_rate != current_ticks {
                            println!("ticks rate change: {} => {}", last_rate, current_ticks);
                            tickrate.store(current_ticks, Ordering::Relaxed);
                            current_ticks = 0;
                        }
                     }
                }
            }
        }
    });

    // aggregate input channels into message channel
    tokio::task::spawn(async move {
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
            }
        }
    });

    // the decision maker
    use popl::util::*;
    use Message::*;

    let mut timeout = time::interval(Duration::from_millis(opts.event_loop_time));
    let mut planter_lowered = limit_planter_lift.read() == Level::Low;
    println!(
        "detected planter {}",
        if planter_lowered { "lowered" } else { "raised" }
    );

    loop {
        let fps = mph_to_fps(ground_speed);
        let target_sps = fps_to_sps(fps, seed_spacing);
        let target_tps = ticks_per_pick() as f32 * target_sps;
        let mph = sps_to_mph(
            seed_per_ticks(tickrate.load(Ordering::Relaxed) as usize) as f32,
            seed_spacing,
        );

        select! {
            Some(msg) = msg_rx.recv() => {
                match msg {
                    GroundSpeed(speed) => {
                        println!("Ground Speed: {speed}");
                        println!("target sps {target_sps} for speed of {speed}");
                        ground_speed = speed;
                    },
                    HopperFull(i) => {
                        println!("hopper {i} full");
                        hopper_relay_pins[i].write(Level::High)
                    },
                    HopperEmpty(i) => {
                        println!("hopper {i} empty");
                        hopper_relay_pins[i].write(Level::Low)
                    },
                    PlanterRaised => {
                        println!("planter raised - dol[{}]", opts.disable_on_lift);
                        // if opts.disable_on_lift {
                        //     planter_lowered = false;
                        //     dc_motor.set_throttle(&mut pwm, -1.0).expect("throttle");
                        //     // todo;; keep an eye on this =============================================
                        //     thread::sleep(Duration::from_secs(2));
                        // } else {
                        //     // dc_motor.set_throttle(&mut pwm, 1.0)?;
                        //     // thread::sleep(Duration::from_secs(2));
                        //     // dc_motor.set_throttle(&mut pwm, 0.0)?;
                        //     // dc_motor.set_throttle(&mut pwm, -1.0)?;
                        //     // thread::sleep(Duration::from_secs(2));
                        //     // dc_motor.set_throttle(&mut pwm, 0.0)?;
                        //     // dc_motor.set_throttle(&mut pwm, 0.5)?;
                        //     // thread::sleep(Duration::from_millis(1750));
                        //     // dc_motor.set_throttle(&mut pwm, 0.0)?;
                        // }
                    },
                    PlanterLowered => {
                        println!("planter lowered");
                        planter_lowered = true;
                    },
                }
            }
            _ = timeout.tick() => {
                use std::io::{self, Write};

                //if planter_lowered {
                if !opts.disable_speed {
                    // automatically adjust the flow control
                    match tickrate.load(Ordering::Relaxed) {
                        tps if (tps as f32) < target_tps => {
                            //print!("+");
                            io::stdout().flush();
                            // increase flow
                            dc_motor.set_throttle(&mut pwm, opts.throttle_rate).expect("throttle +");
                            thread::sleep(Duration::from_millis(opts.throttle_time));
                            dc_motor.set_throttle(&mut pwm, 0.0).expect("throttle + 0.0");
                        }
                        tps if (tps as f32) > target_tps => {
                            //print!("-");
                            io::stdout().flush();
                            // reduce flow
                            dc_motor.set_throttle(&mut pwm, -opts.throttle_rate).expect("throttle -");
                            thread::sleep(Duration::from_millis(opts.throttle_time));
                            dc_motor.set_throttle(&mut pwm, 0.0).expect("throttle - 0.0");
                        }
                        _ => {
                            // hold position
                            //println!(".");
                            io::stdout().flush();
                            dc_motor.set_throttle(&mut pwm, 0.0).expect("throttle . 0.0");
                        }
                    }
                } else {
                    if !opts.quiet{
                        println!("target mph: {mph}");
                    }
                }
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
            //println!("idx {}", encoder_idx);
        }
    }

    Ok(())
}
