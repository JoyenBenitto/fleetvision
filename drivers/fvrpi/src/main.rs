use rppal::gpio::{Gpio, InputPin, Level};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn read_sensor(sensor: Arc<Mutex<InputPin>>) {
    // Function to read sensor data at real-time
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
    // Main function to run sensor mutex op
    let gpio = Gpio::new().unwrap();

    let ir_pin = gpio.get(17).unwrap().into_input(); // IR sensor pin
//    let temp_pin = gpio.get(18).unwrap().into_input(); // Temperature sensor pin

    let ir = Arc::new(Mutex::new(ir_pin));
//    let temp = Arc::new(Mutex::new(temp_pin));

    let ir_thread = thread::spawn(move || {
        read_sensor(ir);
    });

/*    let temp_thread = thread::spawn(move || {
        read_sensor(temp);
    });
*/

    ir_thread.join().unwrap();
//    temp_thread.join().unwrap();
}

