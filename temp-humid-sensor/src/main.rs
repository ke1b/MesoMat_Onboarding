#![no_std]
#![no_main]

//VIN -> P30-P5V0, 3Vo -> (none), GND -> P30-GND, SCL -> P0-04, SDA -> P0-03

use defmt::*;
use embassy_executor::Spawner;
use embassy_nrf as _; // Ensures hardware links properly
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use embassy_nrf::twim::{self, Twim};
use embassy_nrf::{bind_interrupts, peripherals};
use embassy_time::Timer;
use static_cell::ConstStaticCell;
use {defmt_rtt as _, panic_probe as _};

const SI7021_ADDR: u8 = 0x40;
const CMD_RH: u8 = 0xE5;
const CMD_TEMP: u8 = 0xE0;

bind_interrupts!(struct Irqs{
    SERIAL20 => twim::InterruptHandler<peripherals::SERIAL20>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    // Setup
    info!("I2C");
    let config = twim::Config::default();
    static RAM_BUFFER: ConstStaticCell<[u8; 16]> = ConstStaticCell::new([0; 16]);
    let mut i2c = Twim::new(
        p.SERIAL20,
        Irqs,
        p.P1_11,
        p.P1_12,
        config,
        RAM_BUFFER.take(),
    );

    let mut rh_data = [0u8; 3]; //MSB, LSB, Checksum
    let mut temp_data = [0u8; 2]; //MSB, LSB
    info!("Set up complete");
    info!("Reading...");
    loop {
        i2c.write_read(SI7021_ADDR, &[CMD_RH], &mut rh_data)
            .await
            .ok();
        let raw_rh = u16::from_be_bytes([rh_data[0], rh_data[1]]) as f32;
        let humidity = ((125.0 * raw_rh) / 65536.0) - 6.0;
        //info!("{:?}\t->\t{}", rh_data, humidity);

        i2c.write_read(SI7021_ADDR, &[CMD_TEMP], &mut temp_data)
            .await
            .ok();
        let raw_temp = u16::from_be_bytes([temp_data[0], temp_data[1]]) as f32;
        let temp = ((175.72 * raw_temp) / 65536.0) - 46.85;
        //info!("{:?}\t->\t{}", temp_data, temp);
        info!("Temp: {} C\t Humidity: {} %", temp, humidity);
        Timer::after_millis(1000).await;
    }
    //info!("Finished");
}
