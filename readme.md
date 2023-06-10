Potato Plant(er)
===

Monitor and control Harriston n-row potato planter from a Raspberry Pi.

![](doc/6row-planter.png)

https://www.harriston-mayo.com/harriston/equipment-type/potato-planting/

## Hardware
- Raspberry Pi 3
- Official Pi screen
- Adafruit DC motor hat
- Taiss 100 step encoder
- Baomain limit switches in hopper sensors
- Baomain limit switch for lift sensor
- 12v relays
- 12v couplers

## Software
- Tokio
- Rppal gpio
- Iced GUI


## Physical IO

Inputs and outputs can be classified as being planter wide (one per planter) or per-row.

### Planter Inputs
- Planter raised/lowered sensor
- Pick wheel drive shaft sensor
- GPS speed

### Planter Outputs
- Pick wheel flow servo

### Row Inputs
- Hopper fill sensor
- Seed sensor eye

### Row Outputs
- Seed belt control


## Status

1. - [X] Seed belt control
2. - [X] Pick wheel flow control
3. - [X] Pick wheel speed sensor
4. - [X] Planter raised sensor
5. - [X] GPS speed sensor
6. - [X] Hopper fill sensor
7. - [ ] Seed sensor (eye)
8. - [ ] Row context tracking based on planter raised sensor
9. - [ ] J1939 speed sensor
10. - [X] Auto seed wheel speed control
11. - [ ] Seed placement metrics based on seed sensor

Replaced the previous Dickey John monitor and planted 65k row feet with the cli app.

GUI app in progress:

![](doc/wireframe-1.png)


### Required for building
- libfontconfig-dev
- libudev-dev


## Build

```
export PKG_CONFIG_ALLOW_CROSS=1
rustup target add armv7-unknown-linux-gnueabihf
apt install gcc-arm-linux-gnueabihf
cargo build --target=armv7-unknown-linux-gnueabihf
```
