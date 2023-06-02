// Copyright (C) 2016 Felix Obenhuber
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

extern crate nmea;

use std::time::Duration;
use std::{
    env,
    fs::File,
    io,
    io::{BufRead, BufReader},
};

fn main() {
    let port_name = "/dev/ttyACM0";

    let port = serialport::new(port_name, 9600)
        .timeout(Duration::from_millis(10))
        .open();

    match port {
        Ok(mut port) => {
            let mut nmea = nmea::Nmea::default();

            let mut serial_buf: Vec<u8> = vec![0; 1000];
            loop {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => {
                        if t > 0 {
                            nmea.parse(std::str::from_utf8(&serial_buf[..t]).unwrap());
                            println!("{:?}", nmea);
                        } else {
                            break;
                        }

                        //io::stdout().write_all(&serial_buf[..t]).unwrap()
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}
