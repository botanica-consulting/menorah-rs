#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_0, Level::Low);

    let mut button = Input::new(p.PIN_6, Pull::Up);

    loop {
        // Wait for the button to be pressed
        // Turn on the led for 1 second
        button.wait_for_low().await;
        led.set_high();
        Timer::after_secs(1).await;
        led.set_low();
    }
}
