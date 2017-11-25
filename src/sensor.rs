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

pub struct SensorStream {
    timer: Timer,
    sleep: Sleep,
    sample_rate: Duration,
}

impl SensorStream {
    pub fn new(sample_rate: Duration) -> SensorStream {
        let timer = Timer::default();
        let sleep = timer.sleep(sample_rate);
        SensorStream{
            timer: timer,
            sleep: sleep,
            sample_rate: sample_rate,
        }
    }
}

impl Stream for SensorStream {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<()>, io::Error> {
        let _ = try_ready!(self.sleep.poll());
        //println!("{}", self.sample());
        self.sleep = self.sleep.timer().sleep(self.sample_rate);
        Ok(Async::Ready(Some(())))
    }
}

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

#[test]
fn smoke() {
    // Basically if this tests compiles and run we are good.
    let mut t = TempSensor::new("TestSensor");
    let new_rate = Duration::from_millis(1000);
}

#[test]
fn smoke_stream() {
    let dur = Duration::from_secs(1);
    let s = SensorStream::new(dur);
}