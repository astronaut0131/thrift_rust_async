pub fn test() {
    println!("hello");
}


use async_std::{
    io::BufReader,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    prelude::*,
    task,
};
use threadpool::ThreadPool;
use crate::transport::socket::TTcpChannel;
use async_std::io::ErrorKind;
use async_std::io;
use crate::transport::framed::TFramedReadTransport;
use crate::transport::Read;


pub struct TServer {
}

impl TServer {
    pub fn new() -> TServer {
        TServer {
        }
    }

    pub async fn listen<A: ToSocketAddrs>(&mut self, listen_address: A) -> crate::Result<()> {
        println!("into listen");
        let listener = TcpListener::bind(listen_address).await?;

        let mut incoming = listener.incoming();
        println!("begin listening to:");
        while let Some(stream) = incoming.next().await {
            // stream is a new tcp connection stream
            let stream = stream?;
            println!("Accepting from: {}", stream.peer_addr()?);
            // new tcp reader thread

            let channel = TTcpChannel::with_stream(stream);
            task::spawn(handle_incoming_connection(channel));
        }

        Err(io::Error::new(
            ErrorKind::NotConnected,
            "tcp endpoint not connected",
        ))
    }
}


async fn handle_incoming_connection(chan: TTcpChannel) {
    let mut t = TFramedReadTransport::new(chan);
    let mut b = vec![0u8; 10];
    let size = t.read(&mut b).await;

    println!("{:?}, {:?}", size, b);
}