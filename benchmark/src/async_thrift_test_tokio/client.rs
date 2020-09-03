// only for test!!!
//

use std::io::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use futures::AsyncWriteExt;
use time::Duration;
use tokio::{
    net::{TcpStream, ToSocketAddrs},
    task,
};

use async_thrift_tokio::protocol::{TFieldIdentifier, TType};
use async_thrift_tokio::protocol::{TAsyncInputProtocol, TAsyncOutputProtocol};
use async_thrift_tokio::protocol::async_binary::{TAsyncBinaryInputProtocol, TAsyncBinaryOutputProtocol};
use async_thrift_tokio::transport::{AsyncReadHalf, AsyncWrite, AsyncWriteHalf, TAsyncIoChannel};
use async_thrift_tokio::transport::async_buffered::{TAsyncBufferedReadTransport, TAsyncBufferedWriteTransport};
use async_thrift_tokio::transport::async_framed::{TAsyncFramedReadTransport, TAsyncFramedWriteTransport};
use async_thrift_tokio::transport::async_socket::TAsyncTcpChannel;

use crate::async_thrift_test_tokio::tutorial::{CalculatorSyncClient, TCalculatorSyncClient};

pub type Result<T> = std::result::Result<T, Error>;


pub async fn run_client(addr: String, loop_num: i32) -> async_thrift_tokio::Result<(Box<Vec<i64>>)> {
    // time
    // let start = time::now();
    let mut stream = TcpStream::connect(addr.as_str()).await?;

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

    for _ in 0..loop_num {
        let before = time::Instant::now();
        client.ping().await?;
        let end = time::Instant::now();

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