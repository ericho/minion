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
use std::path::{Path, PathBuf};

use walkdir::WalkDir;
use regex::Regex;

pub struct CpuSensor {
    name: String,
}

pub fn sample_interval(dur: Duration, handle: &Handle) -> Box<Future<Item=(), Error=io::Error>>{
    let interval = Interval::new(dur, handle)
        .unwrap();
    let int_stream = interval.for_each(|_| {
        let cpu = CpuSensor::new("Frequency");
        println!("{}", cpu.sample());
        Ok(())
    });

    Box::new(int_stream)
}

impl CpuSensor {
    pub fn new(name: &str) -> CpuSensor {
        CpuSensor{
            name: name.to_string(),
        }
    }

    fn get_metrics(&self, entry: &walkdir::DirEntry) {
        let p = entry.path().to_path_buf();
        println!("The path : {:?}", p)
    }

    fn is_cpu_entry(&self, entry: &walkdir::DirEntry) -> bool {
        let re = Regex::new("cpu[0-9]+").unwrap();
        let name = entry.file_name().to_str().unwrap();
        if re.is_match(name) {
            true
        } else {
            false
        }
    }

}

impl Sensor for CpuSensor {
    fn sample(&self) -> String {
        for entry in WalkDir::new("/sys/bus/cpu/devices")
            .follow_links(true)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok()) {
                if self.is_cpu_entry(&entry) {
                    self.get_metrics(&entry);
                }
            }
        format!("Sampling {} sensor", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_cpu_metrics() {
        let cpu = CpuSensor::new("TestSensor");
        let metrics = cpu.sample();
        assert_eq!("Sampling {} sensor", metrics);
    }
}
