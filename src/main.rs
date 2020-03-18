use rust_gpiozero::*;
use std::thread::sleep;
use std::time::Duration;

const BUTTON_GPIO_PIN: u8 = 2;
const LED_GPIO_PIN: u8 = 18;
const BLINKING_COUNT: u8 = 3;

fn main() {
    let mut button = Button::new(BUTTON_GPIO_PIN);

    loop {
        button.wait_for_press(None);
        blink(LED_GPIO_PIN, BLINKING_COUNT);
    }
}

fn blink(led_pin: u8, count: u8) {
    let led = LED::new(led_pin);

    for _ in 0..count {
        led.on();
        sleep(Duration::from_secs(1));
        led.off();
        sleep(Duration::from_secs(1));
    }
}
