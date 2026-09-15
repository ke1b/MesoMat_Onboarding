#![no_std]
#![no_main]

//VIN -> P30-P5V0, 3Vo -> (none), GND -> P30-GND, SCL -> P1-12, SDA -> P1-11

use defmt::*;
use embassy_executor::Spawner;
use embassy_nrf::twim::{self, Twim};
use embassy_nrf::{bind_interrupts, peripherals, uarte};
use embassy_time::Timer;
use heapless::String;
use static_cell::ConstStaticCell;
use {defmt_rtt as _, panic_probe as _};

const SI7021_ADDR: u8 = 0x40;
const CMD_RH: u8 = 0xE5;
const CMD_TEMP: u8 = 0xE0;

bind_interrupts!(struct Irqs{
    SERIAL20 => twim::InterruptHandler<peripherals::SERIAL20>;
    SERIAL21 => uarte::InterruptHandler<peripherals::SERIAL21>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    // Setup
    info!("I2C Setup");
    let i2c_config = twim::Config::default();
    static RAM_BUFFER: ConstStaticCell<[u8; 16]> = ConstStaticCell::new([0; 16]);
    let mut i2c = Twim::new(
        p.SERIAL20,
        Irqs,
        p.P1_11,
        p.P1_12,
        i2c_config,
        RAM_BUFFER.take(),
    );
    let mut rh_data = [0u8; 3]; //MSB, LSB, Checksum
    let mut temp_data = [0u8; 2]; //MSB, LSB

    info!("UART Setup");
    let mut uart_config = uarte::Config::default();
    uart_config.parity = uarte::Parity::Excluded;
    uart_config.baudrate = uarte::Baudrate::Baud115200;
    let mut uart = uarte::Uarte::new(p.SERIAL21, p.P1_05, p.P1_04, Irqs, uart_config);
    let mut buf = [0; 48];
    let set_up_msg = b"UART set up successfully\r\n";
    buf[..set_up_msg.len()].copy_from_slice(set_up_msg);
    unwrap!(uart.write(&buf).await);
    info!("Set up complete");

    loop {
        // Get humidity
        i2c.write_read(SI7021_ADDR, &[CMD_RH], &mut rh_data)
            .await
            .ok();
        let raw_rh = u16::from_be_bytes([rh_data[0], rh_data[1]]) as f32;
        let humidity = ((125.0 * raw_rh) / 65536.0) - 6.0;
        let mut humidity_buf = ryu::Buffer::new();
        let humidity_str: &str = humidity_buf.format(humidity);

        // Get temperature
        i2c.write_read(SI7021_ADDR, &[CMD_TEMP], &mut temp_data)
            .await
            .ok();
        let raw_temp = u16::from_be_bytes([temp_data[0], temp_data[1]]) as f32;
        let temp = ((175.72 * raw_temp) / 65536.0) - 46.85;
        let mut temp_buf = ryu::Buffer::new();
        let temp_str: &str = temp_buf.format(temp);

        let msg: String<48> = format_output_str(temp_str, humidity_str);
        buf[..msg.len()].copy_from_slice(msg.as_bytes());
        unwrap!(uart.write(&buf[..msg.len()]).await);

        Timer::after_millis(1000).await;
    }
}

fn format_output_str(temp_str: &str, humidity_str: &str) -> String<48> {
    let mut msg: String<48> = String::new();
    msg.push_str("Temp: ").ok();
    msg.push_str(temp_str).ok();
    msg.push_str(" C\tHumidity: ").ok();
    msg.push_str(humidity_str).ok();
    msg.push_str(" %\r\n").ok();
    return msg;
}
