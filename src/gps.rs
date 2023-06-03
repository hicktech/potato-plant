use nmea::SentenceType;
use std::time::Duration;
use std::{io, thread};
use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub enum GroundSpeed {
    Unavailable,
    Gps(f32),
}

pub async fn read_speed(tx: Sender<GroundSpeed>, port_name: &str) -> io::Result<()> {
    let mut port = serialport::new(port_name, 9600)
        .timeout(Duration::from_millis(10))
        .open()?;

    let mut nmea = nmea::Nmea::default();
    let mut serial_buf: Vec<u8> = vec![0; 1000];
    let mut timed_out_counter = 0;
    let mut last_speed = 0.0f32;
    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                if t > 0 {
                    if let Ok(SentenceType::RMC) =
                        nmea.parse(std::str::from_utf8(&serial_buf[..t]).unwrap())
                    {
                        match nmea.speed_over_ground {
                            Some(speed) => {
                                if speed != last_speed {
                                    last_speed = speed;
                                    tx.send(GroundSpeed::Gps(speed)).await;
                                }
                            }
                            None => {
                                tx.send(GroundSpeed::Unavailable).await;
                            }
                        };
                    }
                }
                timed_out_counter = 0;
            }
            Err(e) if e.kind() == io::ErrorKind::TimedOut => {
                timed_out_counter += 1;
                if timed_out_counter > 100 {
                    return Err(e);
                }
                thread::sleep(Duration::from_millis(100));
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
