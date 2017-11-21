extern crate tokio_core;
#[macro_use]
extern crate futures;
extern crate tokio_timer;

use tokio_core::reactor::Core;
//use tokio_core::reactor::timeout_token::TimeoutToken;
use std::time::Duration;
use std::io;
use futures::Future;
use futures::stream::Stream;

mod sensor;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let temp = sensor::TempSensor::new("MySensor");

    let temp_stream = temp.for_each(|_| { 
        println!("Temp!");
        Ok(())
    });
    handle.spawn(temp_stream.map_err(|_| ()));
    core.run(futures::future::empty::<(), ()>()).unwrap();
}
