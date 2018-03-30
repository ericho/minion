extern crate futures;
extern crate tokio_core;

use sensor::Sensor;
use temp_sensor::Metric;

pub struct FreqSensor {
    name: String,
}

impl FreqSensor {
    pub fn new() -> FreqSensor {
        FreqSensor { name: "FreqSensor".to_string() }
    }
}

impl Sensor for FreqSensor {
    fn sample(&self) -> Vec<Metric> {
        Vec::new()
    }
}
