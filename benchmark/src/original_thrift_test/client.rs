// add extern crates here, or in your lib.rs
extern crate ordered_float;
extern crate thrift;
extern crate try_from;

// generated Rust module


use thrift::protocol::{TCompactInputProtocol, TCompactOutputProtocol};
use thrift::protocol::{TInputProtocol, TOutputProtocol};
use thrift::transport::{TFramedReadTransport, TFramedWriteTransport};
use thrift::transport::{TIoChannel, TTcpChannel};

use std::thread;
use crate::original_thrift_test::with_struct::{CalculatorSyncClient, Input, TCalculatorSyncClient};
use std::net::TcpStream;

pub fn run(stream: TcpStream, loop_num : i32) -> thrift::Result<(Box<Vec<i64>>)> {
    //
    // build client
    //

    // println!("connect to server on 127.0.0.1:9090");

    let channel = TTcpChannel::with_stream(stream);

    let mut time_array = Vec::with_capacity(loop_num as usize);

    let (i_chan, o_chan) = channel.split()?;

    let i_prot = TCompactInputProtocol::new(
        TFramedReadTransport::new(i_chan)
    );
    let o_prot = TCompactOutputProtocol::new(
        TFramedWriteTransport::new(o_chan)
    );

    let mut client = CalculatorSyncClient::new(i_prot, o_prot);
    let mut sum = 0;
    for i in 0..loop_num {
        let before = time::now();
        sum += client.add( Input{
            num1: Some(72),
            num2: Some(2),
            comment: None
        })?.res.unwrap();
        let end = time::now();
        time_array.push((end - before).num_nanoseconds().unwrap());
    }

    //
    // println!("final result {}", sum);
    // println!("Test pass, It's time to cheer!");

    // done!
    // println!("finish client");
    Ok((Box::new(time_array)))
}