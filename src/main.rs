// #![feature(global_allocator)]
// #![feature(allocator_api)]

// use std::heap::System;
// #[global_allocator]
// static ALLOCATOR: System = System;

extern crate futures;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate structopt;
extern crate sysinfo;
extern crate tokio;
extern crate tokio_core;
extern crate tokio_io;

use std::net::SocketAddr;

use structopt::StructOpt;
use futures::future;

mod sensor;
mod server;
mod cpu_sensor;
mod temp_sensor;
mod freq_sensor;

static DEFAULT_ADDR: &'static str = "127.0.0.1:6565";

#[derive(StructOpt, Debug)]
#[structopt(name = "minion")]
struct Opt {
    #[structopt(short = "a", long = "aggregator")]
    aggr: Option<String>,
}

fn get_node_future(addr: SocketAddr) -> Box<futures::Future<Item = (), Error = ()> + Send> {
    Box::new(future::lazy(move || {
        sensor::init_sensors(&addr);
        futures::future::empty::<(), ()>()
    }))
}

fn get_aggr_future() -> Box<futures::Future<Item = (), Error = ()> + Send> {
    server::get_server_future()
}

fn main() {
    let opt = Opt::from_args();

    let f = match opt.aggr {
        Some(addr) => {
            let addr = addr.parse::<SocketAddr>().unwrap();
            get_node_future(addr)
        }
        None => get_aggr_future(),
    };

    tokio::run(f);
}
