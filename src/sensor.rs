extern crate futures;
extern crate tokio_core;
extern crate tokio_timer;

pub trait Sensor {
    fn sample(&self) -> String;
}
