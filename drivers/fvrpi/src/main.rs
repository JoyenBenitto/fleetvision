use rppal::gpio::{Gpio, InputPin, Level};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn read_sensor(sensor: Arc<Mutex<InputPin>>) {
    loop {
        if sensor.lock().unwrap().read() == Level::High {
            println!("Obstacle detected on sensor {:?}", sensor);
        } else {
            println!("No obstacle detected on sensor {:?}", sensor);
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn main() {
    let gpio = Gpio::new().unwrap();

    let sensor1_pin = gpio.get(17).unwrap().into_input();
    let sensor2_pin = gpio.get(18).unwrap().into_input();

    let sensor1 = Arc::new(Mutex::new(sensor1_pin));
    let sensor2 = Arc::new(Mutex::new(sensor2_pin));

    let sensor1_thread = thread::spawn(move || {
        read_sensor(sensor1);
    });

    let sensor2_thread = thread::spawn(move || {
        read_sensor(sensor2);
    });

    sensor1_thread.join().unwrap();
    sensor2_thread.join().unwrap();
}

