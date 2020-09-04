// only for test!!!
//

use std::io::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use async_std::{
    net::{TcpListener, TcpStream, ToSocketAddrs},
    task,
};
use async_std::sync::Receiver;
use thrift::transport::TTcpChannel;
use time::Duration;

use async_thrift::protocol::{TFieldIdentifier, TType};
use async_thrift::protocol::async_binary::{TAsyncBinaryInputProtocol, TAsyncBinaryOutputProtocol};
use async_thrift::protocol::TAsyncOutputProtocol;
use async_thrift::transport::{AsyncReadHalf, AsyncWrite, AsyncWriteHalf, TAsyncIoChannel};
use async_thrift::transport::async_buffered::{TAsyncBufferedReadTransport, TAsyncBufferedWriteTransport};
use async_thrift::transport::async_framed::{TAsyncFramedReadTransport, TAsyncFramedWriteTransport};
use async_thrift::transport::async_socket::TAsyncTcpChannel;
use crate::async_thrift_test::echo::{LongMessageTestSyncClient,TLongMessageTestSyncClient};

pub type Result<T> = std::result::Result<T, Error>;


pub async fn run_client(addr: String, loop_num: i32, receiver: Receiver<Vec<i8>>) -> async_thrift::Result<(Box<Vec<i64>>)> {
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

    let mut client = LongMessageTestSyncClient::new(i_prot, o_prot);

    let mut time_array = Vec::with_capacity(loop_num as usize);

    loop {
        let vec = receiver.recv().await.unwrap();
        if vec.len() > 1 {
            let before = time::Instant::now();
            let s = client.echo(vec).await?;
            let end = time::Instant::now();
            time_array.push((end - before).num_nanoseconds().unwrap());
        } else {
            break;
        }
    }

    c.close();

    Ok((Box::new(time_array)))
}