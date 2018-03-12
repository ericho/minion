extern crate futures;
extern crate tokio_core;
extern crate tokio_timer;

use temp_sensor::Metric;

pub trait Sensor {
    fn sample(&self) -> Vec<Metric>;
}
