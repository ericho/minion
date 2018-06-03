// #![feature(global_allocator)]
// #![feature(allocator_api)]

// use std::heap::System;
// #[global_allocator]
// static ALLOCATOR: System = System;

extern crate tokio;
extern crate tokio_io;
extern crate tokio_core;
extern crate futures;
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
use tokio_io::codec::BytesCodec;
use tokio::net::TcpListener;
use tokio::prelude::*;

mod sensor;
mod cpu_sensor;
mod temp_sensor;
mod freq_sensor;

#[derive(StructOpt, Debug)]
#[structopt(name = "minion")]
struct Opt {
    #[structopt(short = "a", long = "aggregator")]
    aggr: String,
    #[structopt(short = "n", long = "node")]
    mode: bool
}

fn get_node_future(addr: SocketAddr) -> Box<futures::Future<Item=(), Error=()> + Send> {
    Box::new(future::lazy(move || {
        sensor::init_sensors(&addr);
        futures::future::empty::<(), ()>()
    }))
}

fn get_aggr_future(addr: SocketAddr) -> Box<futures::Future<Item=(), Error=()> + Send> {
    Box::new(future::lazy(move || {
        let socket = TcpListener::bind(&addr).unwrap();
        println!("Start listening on: {}", addr);
        let done = socket
            .incoming()
            .map_err(|e| println!("Failed to accept socket; error = {:?}", e))
            .for_each(move |socket| {
                let framed = socket.framed(BytesCodec::new());
                let (_writer, reader) = framed.split();

                let processor = reader
                    .for_each(|bytes| {
                        println!("bytes: {:?}", bytes);
                        Ok(())
                    })
                    .and_then(|()| {
                        println!("Socket received FIN packet and closed connection");
                        Ok(())
                    })
                    .or_else(|err| {
                        println!("Socket closed with error: {:?}", err);
                        Err(err)
                    })
                    .then(|result| {
                        println!("Socket close with result: {:?}", result);
                        Ok(())
                    });
                tokio::spawn(processor)
            });
        done
        //futures::future::empty::<(), ()>()
    }))
}


fn main() {
    let opt = Opt::from_args();
    let addr = opt.aggr.parse::<SocketAddr>().unwrap();

    let f = match opt.mode {
        false => get_aggr_future(addr),
        _ => get_node_future(addr),
    };

    tokio::run(f);
}
