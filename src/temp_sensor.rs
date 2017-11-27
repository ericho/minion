
use std::time::{Duration, Instant};
use sensor::{Sensor, SensorStream};


pub struct TempSensor {
    name: String,
    pub stream: SensorStream,
}

impl TempSensor {

    pub fn new(name: &str) -> TempSensor {
        let sample_rate = Duration::from_millis(1000);
        TempSensor{
            name: name.to_string(),
            stream: SensorStream::new(sample_rate),
        }
    }
}

impl Sensor for TempSensor {
    fn sample(&self) -> String {
        "Just a string".to_string()
    }
}