use std::env;
use std::fs::File;
use std::io::Write;

const SENSOR_PARAMETERS_NO: usize = 3;

pub fn get_temp_filepath() -> String {
    //Function to return filepath for temp log
    #[cfg(unix)]
    return "/tmp/mylog.log".into();
}

pub fn log_sensor_data(sensor_array: [f32; SENSOR_PARAMETERS_NO]) {
    let temp_directory = env::temp_dir();
    let temp_file = temp_directory.join("file");

    let mut file = File::create(temp_file).unwrap();
    writeln!(&mut file, "{:?}", sensor_array).unwrap();
    file.write(b"Bytes\n").unwrap();
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
