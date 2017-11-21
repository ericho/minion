extern crate futures;
extern crate tokio_core;
extern crate tokio_timer;

use futures::{Poll, Async, Future};
use futures::stream::Stream;
use tokio_core::reactor::{Remote};
use tokio_timer::{Sleep, Timer};
use std::io;
use std::time::{Duration, Instant};

trait Sensor {
    fn sample(&self) -> String;
}

pub struct TempSensor {
    name: String,
    sample_rate: Duration,
    timer: Timer,
    sleep: Sleep,
}

impl TempSensor {
    pub fn new(name: &str) -> TempSensor {
        let sample_rate = Duration::from_millis(5000);
        let timer = Timer::default();
        let sleep = timer.sleep(sample_rate);
        TempSensor{
            name: name.to_string(),
            sample_rate: sample_rate,
            timer: timer,
            sleep: sleep,
        }
    }

    pub fn new_with_sample_rate(name: &str, sample_rate: Duration) -> TempSensor {
        TempSensor {
            name: name.to_string(),
            sample_rate: sample_rate,
            timer: timer,
            sleep: sleep,
        }
    }

    fn update_sample_rate(&mut self, new_rate: Duration) {
        self.sample_rate = new_rate;
    }
}

impl Sensor for TempSensor {
    fn sample(&self) -> String {
        "Just a string".to_string()
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

#[test]
fn smoke() {
    // Basically if this tests compiles and run we are good.
    let mut t = TempSensor::new("TestSensor");
    let new_rate = Duration::from_millis(1000);
    t.update_sample_rate(new_rate);
}