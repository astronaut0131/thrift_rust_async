// add extern crates here, or in your lib.rs
extern crate ordered_float;
extern crate try_from;

// generated Rust module


use thrift::protocol::{TCompactInputProtocol, TCompactOutputProtocol};
use thrift::protocol::{TInputProtocol, TOutputProtocol};
use thrift::transport::{TFramedReadTransport, TFramedWriteTransport};
use thrift::transport::{TIoChannel, TTcpChannel};

use tutorial::{CalculatorSyncClient, TCalculatorSyncClient};
use tutorial::{Operation, Work};

use async_std::task;

mod tutorial;
mod thrift_server;


async fn run_client(){
    match run() {
        Ok(()) => println!("client ran successfully"),
        Err(e) => {
            println!("client failed with {:?}", e);
            std::process::exit(1);
        }
    }
}

fn main() {
    task::spawn(run_client());
    thrift_server::run();
}

fn run() -> thrift::Result<()> {
    //
    // build client
    //

    println!("connect to server on 127.0.0.1:9090");
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

    //
    // alright! - let's make some calls
    //

    // two-way, void return
    client.ping()?;

    // two-way with some return
    let res = client.add(
        72,
        2
    )?;
    println!("multiplied 72 and 2, got {}", res);

    // match res {
    //     Ok(v) => panic!("shouldn't have succeeded with result {}", v),
    //     Err(e) => println!("divide by zero failed with {:?}", e),
    // }

    // one-way
    client.zip()?;

    // done!
    Ok(())
}