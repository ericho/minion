extern crate futures;
extern crate tokio_core;
extern crate tokio;
extern crate serde_json;

use futures::Future;
use futures::stream::Stream;
use tokio::prelude::*;
use tokio::net::TcpStream;

use tokio::timer::Interval;
use temp_sensor::*;
use freq_sensor::FreqSensor;
use cpu_sensor::CpuSensor;

use std::time::{Duration, Instant};
use std::net::SocketAddr;

pub trait Sensor {
    fn sample(&self) -> Vec<Metric>;
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Metric {
    Temperature(TempMetric),
    Processor(ProcessorMetric),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TempMetric {
    label: String,
    max: f32,
    critical: Option<f32>,
    temperature: f32,
}

impl TempMetric {
    pub fn new(label: String,
               max: f32,
               critical: Option<f32>,
               temperature: f32) -> TempMetric {
        TempMetric {
            label: label,
            max: max,
            critical: critical,
            temperature: temperature,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessorMetric {
    name: String,
    usage: f32,
}

impl ProcessorMetric {
    pub fn new(name: String, usage: f32) -> ProcessorMetric {
        ProcessorMetric {
            name: name,
            usage: usage,
        }
    }
}

fn create_interval<S: Sensor + 'static>(sensor: S,
                                        dur: Duration,
                                        addr: &SocketAddr)
                                        -> impl Future<Item = (), Error = ()> {
    let addr = addr.clone();
    let interval = Interval::new(Instant::now(), dur);

    let interval_stream = interval.for_each(move |_| {
        let sample = sensor.sample();
        let json = serde_json::to_string(&sample).unwrap();

        let tcp = TcpStream::connect(&addr);
        tokio::spawn(
            // TODO: Log errors if cannot send data to the aggregator
            tcp.map(move |mut s| {
                if let Err(e) = s.write(json.as_bytes()) {
                    println!("Error writing data: {:?}", e);
                }
                ()
            }).map_err(|e| println!("Error sending: {:?}", e))
        );

        Ok(())
    }).map_err(|_| ());

    interval_stream
}

// This method will register all the sensors into the tokio executor.
pub fn init_sensors(addr: &SocketAddr) {

    let temp = TempSensor::new();
    let temp_interval = create_interval(temp,
                                        Duration::from_millis(500),
                                        &addr);
    tokio::spawn(temp_interval);

    let freq = FreqSensor::new();
    let freq_interval = create_interval(freq,
                                        Duration::from_millis(500),
                                        &addr);
    //tokio::spawn(freq_interval);

    let cpu = CpuSensor::new();
    let cpu_interval = create_interval(cpu,
                                       Duration::from_millis(500),
                                       &addr);
    tokio::spawn(cpu_interval);

}
