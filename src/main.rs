// #![feature(global_allocator)]
// #![feature(allocator_api)]

// use std::heap::System;
// #[global_allocator]
// static ALLOCATOR: System = System;

extern crate tokio;
extern crate tokio_core;
extern crate futures;
extern crate walkdir;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate sysinfo;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt;

use std::net::SocketAddr;

use structopt::StructOpt;
use futures::future;


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

    let f = future::lazy(move || {
        sensor::init_sensors(&addr);
        futures::future::empty::<(), ()>()
    });

    tokio::run(f);
}
