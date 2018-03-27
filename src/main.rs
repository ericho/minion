#![feature(global_allocator)]
#![feature(allocator_api)]

use std::heap::System;
#[global_allocator]
static ALLOCATOR: System = System;

extern crate tokio;
extern crate tokio_core;
extern crate futures;
extern crate walkdir;
extern crate regex;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt;

// extern crate structopt_derive;

use std::time::Duration;
use std::net::SocketAddr;

use tokio_core::reactor::Core;
use structopt::StructOpt;

mod sensor;
mod cpu_sensor;
mod temp_sensor;
mod freq_sensor;

#[derive(StructOpt, Debug)]
#[structopt(name = "minion")]
struct Opt {
    #[structopt(short = "a", long = "aggregator")]
    aggr: String,
}

fn main() {
    let opt = Opt::from_args();
    let addr = opt.aggr.parse::<SocketAddr>().unwrap();
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let temp_stream = temp_sensor::sample_interval(
        Duration::from_millis(500), &handle, &addr);
    handle.spawn(temp_stream);

    // Wait forever
    core.run(futures::future::empty::<(), ()>()).unwrap();
}
