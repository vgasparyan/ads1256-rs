//! Example to communicate from Raspberry PI with ADS1256 ADC board 
//!
//! The High-Precision AD/DA board was used for testing.
//! [AD/DA board ]https://www.waveshare.com/wiki/High-Precision_AD/DA_Board
//!
extern crate ads1256_rs;
extern crate linux_embedded_hal as linux_hal;

use linux_hal::spidev::{self, SpidevOptions};
use linux_hal::sysfs_gpio::Direction;
use linux_hal::{Delay, Pin, Spidev};

use ads1256_rs::{Channel, Config, Register, SamplingRate, ADS1256, PGA};

use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello ADS1256 driver..");

    let mut spi = Spidev::open("/dev/spidev0.1").unwrap();
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(1500_000)
        .lsb_first(false)
        .mode(spidev::SPI_MODE_1)
        .build();
    spi.configure(&options).unwrap();

    //Output pin
    let cs_pin = Pin::new(22);
    cs_pin.export().unwrap();
    while !cs_pin.is_exported() {}
    cs_pin.set_direction(Direction::Out).unwrap();
    cs_pin.set_value(0).unwrap();

    //Output pin
    let rst_pin = Pin::new(18);
    rst_pin.export().unwrap();
    while !rst_pin.is_exported() {}
    rst_pin.set_direction(Direction::Out).unwrap();

    //Input pin
    let drdy_pin = Pin::new(17);
    drdy_pin.export().unwrap();
    while !drdy_pin.is_exported() {}
    drdy_pin.set_direction(Direction::In).unwrap();

    //reset the adc
    rst_pin.set_value(0).unwrap();
    thread::sleep(Duration::from_micros(1)); //t16 delay (0.52us)
    rst_pin.set_value(1).unwrap();

    //wait for setup
    thread::sleep(Duration::from_millis(200));

    //create driver instance
    let mut adc = ADS1256::new(spi, cs_pin, rst_pin, drdy_pin, Delay).unwrap();
    let config = Config::new(SamplingRate::Sps30000, PGA::Gain2);
    adc.set_config(&config).unwrap();

    ///read all single ended channels in one-shot mode
    for ch in &[Channel::AIN0, Channel::AIN1, Channel::AIN2, Channel::AIN3,
                Channel::AIN4, Channel::AIN5, Channel::AIN6, Channel::AIN7] {

        let code = adc.read_channel(*ch, Channel::AINCOM).unwrap();
        let in_volt = adc.convert_to_volt(code);
        println!("Channel {:?} : {:#08x}, {:.20} V ", ch, code, in_volt);
    }
}
