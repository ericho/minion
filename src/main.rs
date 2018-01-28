// #![feature(global_allocator)]
// #![feature(allocator_api)]

// use std::heap::System;
// #[global_allocator]
// static ALLOCATOR: System = System;

extern crate tokio_core;
extern crate futures;
extern crate futures_cpupool;
extern crate tokio_timer;
extern crate walkdir;
extern crate regex;
extern crate zmq;

mod sensor;
mod cpu_sensor;
mod temp_sensor;
mod freq_sensor;

use tokio_core::reactor::Core;
use futures::Future;
use futures_cpupool::CpuPool;
use std::time::Duration;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let pool = CpuPool::new_num_cpus();

    let temp_stream = temp_sensor::sample_interval(Duration::from_millis(500), &handle, &pool);
    let freq_stream = freq_sensor::sample_interval(Duration::from_millis(1000), &handle);
    handle.spawn(temp_stream.map_err(|_| ()));
    handle.spawn(freq_stream.map_err(|_| ()));

    core.run(futures::future::empty::<(), ()>()).unwrap();
}
