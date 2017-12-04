extern crate tokio_core;
extern crate futures;
extern crate tokio_timer;
extern crate walkdir;
extern crate regex;

mod sensor;
mod cpu_sensor;
mod temp_sensor;
mod freq_sensor;

use tokio_core::reactor::Core;
use futures::Future;
use std::time::Duration;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();


    let temp_stream = temp_sensor::sample_interval(Duration::from_millis(500), &handle);
    let freq_stream = freq_sensor::sample_interval(Duration::from_millis(1000), &handle);
    let cpu_stream = cpu_sensor::sample_interval(Duration::from_millis(1000), &handle);
    handle.spawn(temp_stream.map_err(|_| ()));
    handle.spawn(freq_stream.map_err(|_| ()));
    handle.spawn(cpu_stream.map_err(|_| ()));

    core.run(futures::future::empty::<(), ()>()).unwrap();
}
