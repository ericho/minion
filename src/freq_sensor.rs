extern crate futures;
extern crate tokio_core;
extern crate tokio_timer;

use std::time::Duration;
use sensor::Sensor;
use futures::Future;
use futures::stream::Stream;
use tokio_core::reactor::{Handle, Interval};
use std::io;


pub struct FreqSensor {
    name: String,
}

pub fn sample_interval(dur: Duration,
                       handle: &Handle)
                       -> Box<Future<Item = (), Error = io::Error>> {
    let interval = Interval::new(dur, handle).unwrap();
    let int_stream = interval.for_each(|_| {
        let freq = FreqSensor::new("Frequency");
        println!("{}", freq.sample());
        Ok(())
    });

    Box::new(int_stream)
}

impl FreqSensor {
    pub fn new(name: &str) -> FreqSensor {
        FreqSensor { name: name.to_string() }
    }
}

impl Sensor for FreqSensor {
    fn sample(&self) -> String {
        format!("Sampling {} sensor", self.name)
    }
}
