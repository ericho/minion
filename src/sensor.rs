extern crate futures;
extern crate tokio_core;

use temp_sensor::Metric;

pub trait Sensor {
    fn sample(&self) -> Vec<Metric>;
}
