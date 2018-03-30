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

    sensor::init_sensors(&handle, &addr);

    // Wait forever
    core.run(futures::future::empty::<(), ()>()).unwrap();
}
