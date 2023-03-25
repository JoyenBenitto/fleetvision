use std::env;
use std::fs::File;
use std::io::Write;

const SENSOR_PARAMETERS_NO: usize = 2;

pub fn get_temp_filepath() -> String {
    //Function to return filepath for temp log
    #[cfg(unix)]
    return "/tmp/mylog.log".into();
}

/*pub fn log_sensor_data(sensor_array: [f32; SENSOR_PARAMETERS_NO]) {
    let mut file_ref = std::fs::File::create("mylog.log").expect("create failed");
    file_ref
        .write_all(&sensor_array[0..(SENSOR_PARAMETERS_NO - 1)].as_str())
        .expect("write failed");
}*/

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
