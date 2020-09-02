// only for test!!!
//

use std::time::{SystemTime, UNIX_EPOCH};
use async_std::{
    net::{TcpListener, TcpStream, ToSocketAddrs},
    task,
};
use std::io::Error;

pub type Result<T> = std::result::Result<T, Error>;


use async_thrift::transport::async_socket::TAsyncTcpChannel;
use async_thrift::transport::async_framed::{TAsyncFramedWriteTransport, TAsyncFramedReadTransport};
use async_thrift::transport::{AsyncWrite, TAsyncIoChannel, AsyncReadHalf, AsyncWriteHalf};
use async_thrift::protocol::{TFieldIdentifier, TType};
use async_thrift::protocol::async_binary::{TAsyncBinaryOutputProtocol, TAsyncBinaryInputProtocol};
use async_thrift::protocol::TAsyncOutputProtocol;
use async_thrift::transport::async_buffered::{TAsyncBufferedReadTransport, TAsyncBufferedWriteTransport};
use time::Duration;
use futures::AsyncWriteExt;
use crate::async_thrift_test::with_struct::{CalculatorSyncClient, Input, TCalculatorSyncClient};
use thrift::transport::TTcpChannel;

pub async fn run_client(addr: impl ToSocketAddrs, loop_num: i32) -> async_thrift::Result<(Box<Vec<i64>>)> {
    // time
    // let start = time::now();

    let mut stream = TcpStream::connect(addr).await?;

    let mut c = TAsyncTcpChannel::with_stream(stream);

    let (i_chan, o_chan) = c.split().unwrap();

    let i_prot = TAsyncBinaryInputProtocol::new(
        TAsyncBufferedReadTransport::new(i_chan), true,
    );
    let o_prot = TAsyncBinaryOutputProtocol::new(
        TAsyncBufferedWriteTransport::new(o_chan), true,
    );

    let mut client = CalculatorSyncClient::new(i_prot, o_prot);

    let mut time_array = Vec::with_capacity(loop_num as usize);

    let mut sum = 0;
    for i in 0..loop_num {
        let before = time::now();
        let r = client.add(
            Input{
                num1: Some(1),
                num2: Some(2),
                comment: None
            }
        ).await?;
        let end = time::now();
        sum += r.res.unwrap();
        time_array.push((end - before).num_nanoseconds().unwrap());
    }

    c.close();

    Ok((Box::new(time_array)))
}


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