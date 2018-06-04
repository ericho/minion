extern crate tokio;

use std::net::SocketAddr;

use futures::Future;
use tokio::net::TcpListener;
use tokio_io::codec::BytesCodec;
use tokio::prelude::*;

use super::DEFAULT_ADDR;

pub fn get_server_future() -> Box<Future<Item = (), Error = ()> + Send> {
    let addr = DEFAULT_ADDR.parse::<SocketAddr>().unwrap();
    let socket = TcpListener::bind(&addr).unwrap();
    println!("Start listening on: {}", DEFAULT_ADDR);
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
    Box::new(done)
}
