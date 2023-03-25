use dirs::data_local_dir;
use gag::Redirect;
use rppal::gpio::{Gpio, InputPin, Level};
use rppal::i2c::I2c;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::thread;
use std::time::Duration;

pub mod utils;

const IR_SENSOR_PIN: u8 = 17;
const I2C_DEV_ADDRESS: u16 = 0x5A;
const AMB_TEMP_REG: u8 = 0x06;
const OBJ_TEMP_REG: u8 = 0x07;

fn read_prox_sensor(pin: &InputPin) -> f32 {
    // Function to read IR sensor data at real-time

    let mut duration = Duration::from_secs(0);
    let mut prev_time = std::time::Instant::now();
    let mut pulse_started = false;
    let mut pulse_ended = false;

    let mut last_level = Level::High;

    while !pulse_ended {
        let level = pin.read();
        if level == Level::Low && last_level == Level::High {
            pulse_started = true;
            prev_time = std::time::Instant::now();
        } else if level == Level::High && last_level == Level::Low {
            pulse_ended = true;
        }

        let elapsed = std::time::Instant::now().duration_since(prev_time);
        if pulse_started && elapsed > Duration::from_secs(2) {
            pulse_ended = true;
        }

        if !pulse_ended {
            duration += elapsed;
        }
        last_level = level;
    }

    let duration_micros = duration.as_micros() as f32;
    let distance_cm = duration_micros / 29.0 / 2.0;
    distance_cm
}

fn display_sensor_data() {
    // Function to read temperature sensor data at real-time
    let gpio = Gpio::new().unwrap();
    let mut i2c = I2c::new().unwrap();

    i2c.set_slave_address(I2C_DEV_ADDRESS).unwrap();
    let _ir_pin = gpio.get(IR_SENSOR_PIN).unwrap().into_input_pullup();

    let mut inc = 1;

    loop {
        let ambient_temp = read_temperature(&mut i2c, AMB_TEMP_REG);
        let object_temp = read_temperature(&mut i2c, OBJ_TEMP_REG);
        let ir_distance = read_prox_sensor(&_ir_pin);

        println!("{:?}", inc);
        inc += 1;
        println!("Ambient temperature: {:.2}C", ambient_temp);
        println!("Object temperature: {:.2}C", object_temp);
        println!("IR Distance: {:.2} cm", ir_distance);

        thread::sleep(Duration::from_millis(500));
    }
}

fn read_temperature(i2c: &mut I2c, register: u8) -> f32 {
    // Function to read and parse temperature
    let mut buffer = [0u8; 3];
    i2c.write(&[register]).unwrap();
    thread::sleep(Duration::from_micros(500));
    i2c.read(&mut buffer).unwrap();
    let temperature_data = u16::from_be_bytes([buffer[0], buffer[1]]) >> 2;
    let temperature = (temperature_data as f32) * 0.02 - 273.15;
    temperature
}

fn main() {
    // Main function to run stuff

    let log = OpenOptions::new()
        .truncate(true)
        .read(true)
        .create(true)
        .write(true)
        .open(utils::get_temp_filepath())
        .unwrap();
    utils::print_type_of(&log);
    display_sensor_data();
}
