# Button LED Blinks
> A simple application that will make a LED blink once a push button is pressed

## Getting started

### Prerequisites
- [Docker](https://www.docker.com/) on your development machine
- [Raspberry Pi](https://www.raspberrypi.org/) to run the application

### Hardware setup

Refer to the [Raspberry Pi GPIO Documentation](https://www.raspberrypi.org/documentation/usage/gpio/) for numbering.

1. Connect a push button that goes from pin `GPIO 2` on one end and a `GND` on the other.
2. As a separate circuit, wire a 220 ohms resistor and a LED in series connected to `GPIO 18` and a `GND`

### Option 1: On a local machine

1. Run `make build`
2. Grab the file at `./target/armv7-unknown-linux/gnueabihf/release/button_led_blinks` and copy it to your Raspberry Pi
3. Run the file from the Pi

### Option 2: On the Pi
1. Install [Rust](https://www.rust-lang.org/) on your Rapberry Pi
2. Clone the code to your Raspberry Pi
3. Run `cargo run` or `cargo build`
4. Run the file if you decided to build
