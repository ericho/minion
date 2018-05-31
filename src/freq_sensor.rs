extern crate futures;
extern crate tokio_core;

use sensor::{Metric, Sensor};

pub struct FreqSensor {}

impl FreqSensor {
    pub fn new() -> FreqSensor {
        FreqSensor {}
    }
}

impl Sensor for FreqSensor {
    fn sample(&self) -> Vec<Metric> {
        Vec::new()
    }
}
