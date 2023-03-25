use dirs::data_local_dir;
use gag::Redirect;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};

pub fn get_temp_filepath() -> String {
    //Function to return filepath for temp log
    #[cfg(unix)]
    return "/tmp/mylog.log".into();
}

pub fn log_sensor_data(log: File) {
    // Function to log output sensor data
    let print_redirect = Redirect::stdout(log).unwrap();
    let mut log = print_redirect.into_inner();
    let mut buf = String::new();

    log.seek(SeekFrom::Start(0)).unwrap();
    log.read_to_string(&mut buf).unwrap();
    assert_eq!(&buf[..], "Hidden\n");
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
