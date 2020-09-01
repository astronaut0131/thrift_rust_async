// only for test!!!
//

use std::time::{SystemTime, UNIX_EPOCH};
use async_std::{
    net::{TcpListener, TcpStream, ToSocketAddrs},
    task,
};
use async_std::io::Error;

pub type Result<T> = std::result::Result<T, Error>;


use async_thrift::transport::async_socket::TAsyncTcpChannel;
use async_thrift::transport::async_framed::{TAsyncFramedWriteTransport, TAsyncFramedReadTransport};
use async_thrift::transport::{AsyncWrite, TAsyncIoChannel};
use async_thrift::protocol::{TFieldIdentifier, TType};
use async_thrift::protocol::async_binary::{TAsyncBinaryOutputProtocol, TAsyncBinaryInputProtocol};
use async_thrift::protocol::TAsyncOutputProtocol;
use async_thrift::transport::async_buffered::{TAsyncBufferedReadTransport, TAsyncBufferedWriteTransport};
use time::Duration;
use futures::AsyncWriteExt;
use crate::async_thrift_test::with_struct::{Material, Operator, Number, CalculatorServiceSyncClient, TCalculatorServiceSyncClient,type_a,type_b};


// test transport
pub async fn try_run(addr: impl ToSocketAddrs) -> Result<()> {
    let stream = TcpStream::connect(addr).await?;
    let c = TAsyncTcpChannel::with_stream(stream);
    let mut t = TAsyncFramedWriteTransport::new(c);

    t.write(&[0x00, 0x00, 0x00, 0x02, 0x02, 0x01]).await?;
    t.flush().await?;
    Ok(())
}

// test protocol
pub async fn try_run_protocol(addr: impl ToSocketAddrs) -> Result<()> {
    let stream = TcpStream::connect(addr).await?;
    let mut channel = TAsyncTcpChannel::with_stream(stream);

    let t = TAsyncFramedWriteTransport::new(channel);
    let mut protocol = TAsyncBinaryOutputProtocol::new(t, true);

    protocol.write_field_begin(&TFieldIdentifier::new("string_thing", TType::String, 1)).await.unwrap();
    protocol.write_string("foo").await.unwrap();
    protocol.write_field_end().await.unwrap();
    protocol.flush().await;

    Ok(())
}

// test client
pub async fn run_client(addr: impl ToSocketAddrs, loop_num: i32) -> async_thrift::Result<(Box<Vec<i64>>)> {
    // time
    // let start = time::now();
    let mut time_array = Vec::with_capacity(loop_num as usize);

    let mut stream = TcpStream::connect(addr).await?;
    // println!("{:?}", stream.local_addr());

    let mut c = TAsyncTcpChannel::with_stream(stream);

    let (i_chan, o_chan) = c.split()?;

    let i_prot = TAsyncBinaryInputProtocol::new(
        TAsyncFramedReadTransport::new(i_chan), true,
    );
    let o_prot = TAsyncBinaryOutputProtocol::new(
        TAsyncFramedWriteTransport::new(o_chan), true,
    );

    let mut client = CalculatorServiceSyncClient::new(i_prot, o_prot);
    let mut sum:i64 = 0;
    for i in 0..loop_num {
        let before = time::now();
        let material = Material{
            num1: Some(Number::B(1)),
            num2: Some(Number::B(2)),
            op: Some(Operator::Add)
        };
        let r = client.calculate(
            material
        ).await?;
        match r{
            Number::A(i) => {
                sum += i as i64;
            }
            Number::B(i) => {
                sum += i
            }
        }
        let end = time::now();
        time_array.push((end - before).num_nanoseconds().unwrap());
    }

    c.close();


    // println!("done! duration:{:?} ms", (end - start).num_milliseconds());

    // println!("final result {}", sum);
    // println!("Test pass, It's time to cheer!");

    // println!("finish client");
    Ok((Box::new(time_array)))
}

