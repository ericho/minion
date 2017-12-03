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
}

impl Sensor for CpuSensor {
    fn sample(&self) -> String {
        for entry in WalkDir::new("/sys/bus/cpu/devices")
            .follow_links(true)
            .max_depth(1) {
                println!("Got {:?}", entry);
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
