
extern crate futures;
extern crate tokio_core;
extern crate tokio_timer;

use std::time::{Duration, Instant};
use sensor::{Sensor, SensorStream};
use futures::{Poll, Async, Future};
use futures::stream::Stream;
use tokio_core::reactor::{Remote};
use tokio_timer::{Sleep, Timer};
use std::io;


pub struct TempSensor {
    name: String,
    timer: Timer,
    sleep: Sleep,
    sample_rate: Duration,
}

impl TempSensor {
    pub fn new(name: &str) -> TempSensor {
        let sample_rate = Duration::from_millis(1000);
        let timer = Timer::default();
        let sleep = timer.sleep(sample_rate);
        TempSensor{
            name: name.to_string(),
            timer: timer,
            sleep: sleep,
            sample_rate: sample_rate,
        }
    }
}

impl Stream for TempSensor {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<()>, io::Error> {
        let _ = try_ready!(self.sleep.poll());
        println!("{}", self.sample());
        self.sleep = self.sleep.timer().sleep(self.sample_rate);
        Ok(Async::Ready(Some(())))
    }
}

impl Sensor for TempSensor {
    fn sample(&self) -> String {
        "Just a string".to_string()
    }
}