#![no_std]
#![no_main]

mod fmt;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive};
use embassy_time::Timer;
use fmt::info;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let mut led = Output::new(p.P0_21, Level::Low, OutputDrive::Standard);
    let mut _gnd = Output::new(p.P0_28, Level::Low, OutputDrive::Standard);

    // p.P0_14 as input: btn1
    let btn = Input::new(p.P0_14, embassy_nrf::gpio::Pull::Up);
    let mut counter = 0;

    loop {
        if btn.is_low() {
            counter += 1;
            info!("Button1 pressed {}", counter);
            Timer::after_millis(200).await;
        }
        if counter % 2 == 0 {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
