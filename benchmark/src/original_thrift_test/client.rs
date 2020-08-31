// add extern crates here, or in your lib.rs
extern crate ordered_float;
extern crate thrift;
extern crate try_from;

// generated Rust module


use thrift::protocol::{TCompactInputProtocol, TCompactOutputProtocol};
use thrift::protocol::{TInputProtocol, TOutputProtocol};
use thrift::transport::{TFramedReadTransport, TFramedWriteTransport};
use thrift::transport::{TIoChannel, TTcpChannel};

use crate::original_thrift_test::tutorial::{CalculatorSyncClient, TCalculatorSyncClient};
use std::thread;

pub fn run(loop_num : i32) -> thrift::Result<()> {
    //
    // build client
    //

    // println!("connect to server on 127.0.0.1:9090");

    let mut c = TTcpChannel::new();
    c.open("127.0.0.1:9090")?;

    let (i_chan, o_chan) = c.split()?;

    let i_prot = TCompactInputProtocol::new(
        TFramedReadTransport::new(i_chan)
    );
    let o_prot = TCompactOutputProtocol::new(
        TFramedWriteTransport::new(o_chan)
    );

    let mut client = CalculatorSyncClient::new(i_prot, o_prot);
    let mut sum = 0;
    for i in 0..loop_num {
        sum += client.add(
            72,
            2,
        )?;
    }


    //
    // println!("final result {}", sum);
    // println!("Test pass, It's time to cheer!");

    // done!
    Ok(())
}