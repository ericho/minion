extern crate futures;
extern crate tokio_core;
extern crate tokio;
extern crate serde_json;

use tokio_core::reactor::{Handle, Interval};
use futures::Future;
use futures::stream::Stream;
use tokio::net::TcpStream;
use tokio::prelude::*;
use temp_sensor::*;
use freq_sensor::FreqSensor;
use cpu_sensor::CpuSensor;

use std::time::Duration;
use std::net::SocketAddr;

pub trait Sensor {
    fn sample(&self) -> Vec<Metric>;
}

fn create_interval<S: Sensor + 'static>(sensor: S,
                                        dur: Duration,
                                        handle: &Handle,
                                        addr: &SocketAddr)
                   -> Box<Future<Item = (), Error = ()>> {
    let addr = addr.clone();
    let interval = Interval::new(dur, handle).unwrap();

    let interval_stream = interval.for_each(move |_| {
        let sample = sensor.sample();
        println!("Creating");
        let json = serde_json::to_string(&sample).unwrap();

        let tcp = TcpStream::connect(&addr);

        tokio::spawn(
            tcp
                .map(move |mut s| {
                    s.write(json.as_bytes());
                    ()})
                .map_err(|_| ())
        );

        futures::future::ok(())
    }).map_err(|_| ());

    Box::new(interval_stream)
}

// This method will register all the sensors into the tokio executor.
pub fn init_sensors(handle: &Handle, addr: &SocketAddr) {
    let temp = TempSensor::new();
    let temp_interval = create_interval(temp,
                                        Duration::from_millis(500),
                                        handle,
                                        &addr);
    handle.spawn(temp_interval);

    let freq = FreqSensor::new();
    let freq_interval = create_interval(freq,
                                        Duration::from_millis(500),
                                        handle,
                                        &addr);
    handle.spawn(freq_interval);

    let cpu = CpuSensor::new();
    let cpu_interval = create_interval(cpu,
                                       Duration::from_millis(500),
                                       handle,
                                       &addr);
    handle.spawn(cpu_interval);
}
