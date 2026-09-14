#![no_std]
#![no_main]

//VIN -> P30-P5V0, 3Vo -> (none), GND -> P30-GND, SCL -> P0-04, SDA -> P0-03

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_nrf as _; // Ensures hardware links properly
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _}; // Fixes: missing #[panic_handler]

#[embassy_executor::task(pool_size = 4)]
async fn button_task(n: usize, mut pin: Input<'static>) {
    loop {
        pin.wait_for_low().await;
        info!("pressed");
        pin.wait_for_high().await;
        info!("released");
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello, world!");
    let p = embassy_nrf::init(Default::default());
    //    let mut led = Output::new(p.P2_09, Level::Low, OutputDrive::Standard);
    let btn1 = Input::new(p.P1_13, Pull::Up);

    _spawner.spawn(unwrap!(button_task(1, btn1)));

    /*    loop {
            led.set_high();
            Timer::after_millis(100).await;
            led.set_low();
            Timer::after_millis(100).await;
        }
    */
}
