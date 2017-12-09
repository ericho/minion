extern crate futures;
extern crate tokio_core;
extern crate tokio_timer;
extern crate walkdir;

use std::time::Duration;
use sensor::Sensor;
use futures::Future;
use futures::stream::Stream;
use tokio_core::reactor::{Handle, Interval};
use std::io;
use walkdir::WalkDir;
use std::path::Path;
use regex::Regex;


pub struct TempSensor {
    match_temp_file: Regex,
    match_hwmon_entry: Regex,
}

pub fn sample_interval(dur: Duration, handle: &Handle) -> Box<Future<Item=(), Error=io::Error>>{
    let interval = Interval::new(dur, handle)
        .unwrap();
    let int_stream = interval.for_each(|_| {
        let temp = TempSensor::new();
        println!("{}", temp.sample());
        Ok(())
    });

    Box::new(int_stream)
}

impl TempSensor {
    pub fn new() -> TempSensor {
        TempSensor{
            match_temp_file: Regex::new("temp[0-9]+_(input|label)").unwrap(),
            match_hwmon_entry: Regex::new("hwmon[0-9]+").unwrap(),

        }
    }

    fn is_hwmon_entry(&self, entry: &Path) -> bool {
        self.match_hwmon_entry.is_match(entry.to_str().unwrap())
    }

    fn is_temp_file(&self, p: &Path) -> bool {
        self.match_temp_file.is_match(p.to_str().unwrap())
    }
}

impl Sensor for TempSensor {
    fn sample(&self) -> String {
        for entry in WalkDir::new("/sys/class/hwmon")
            .follow_links(true)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| self.is_hwmon_entry(e.path())) {
                if self.is_temp_file(entry.path()) {
                    println!("{:?}", entry);
                }
            }
        format!("Sampling temp sensor")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_temp_metrics() {
        let temp = TempSensor::new();
        let _samples = temp.sample();
    }

    #[test]
    fn check_if_folder_is_hwmon() {
        let temp = TempSensor::new();
        let p = Path::new("/sys/class/hwmon");
        assert_eq!(temp.is_hwmon_entry(&p), false);
        let p = Path::new("/some/bad/path");
        assert_eq!(temp.is_hwmon_entry(&p), false);
        let p = Path::new("/sys/class/hwmon0");
        assert_eq!(temp.is_hwmon_entry(&p), true);
        let p = Path::new("/sys/class/hwmon10000");
        assert_eq!(temp.is_hwmon_entry(&p), true);
    }

    #[test]
    fn check_if_file_is_temp_file() {
        let temp = TempSensor::new();
        let p = Path::new("/sys/class/hwmon/hwmon0/temp1_input");
        assert_eq!(temp.is_temp_file(p), true);
        let p = Path::new("/sys/class/hwmon/hwmon0/temp1_label");
        assert_eq!(temp.is_temp_file(p), true);
        let p = Path::new("/sys/class/hwmon/hwmon0/tempA_input");
        assert_eq!(temp.is_temp_file(p), false);
    }
}
