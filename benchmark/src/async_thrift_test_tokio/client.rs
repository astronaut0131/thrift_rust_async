// only for test!!!
//

use std::time::{SystemTime, UNIX_EPOCH};
use tokio::{
    net::{TcpStream, ToSocketAddrs},
    task,
};
use std::io::Error;
pub type Result<T> = std::result::Result<T, Error>;
use async_thrift_tokio::transport::async_socket::TAsyncTcpChannel;
use async_thrift_tokio::transport::async_framed::{TAsyncFramedWriteTransport, TAsyncFramedReadTransport};
use async_thrift_tokio::transport::{AsyncWrite, TAsyncIoChannel};
use async_thrift_tokio::protocol::{TFieldIdentifier, TType};
use async_thrift_tokio::protocol::async_binary::{TAsyncBinaryOutputProtocol, TAsyncBinaryInputProtocol};
use async_thrift_tokio::protocol::TAsyncOutputProtocol;
use async_thrift_tokio::transport::async_buffered::{TAsyncBufferedReadTransport, TAsyncBufferedWriteTransport};
use time::Duration;
use futures::AsyncWriteExt;
use crate::async_thrift_test_tokio::with_list_map::{ListMapTestSyncClient,TListMapTestSyncClient};
use std::collections::BTreeMap;
use crate::THREAD_NUM;

// test client
pub async fn run_client(addr: impl ToSocketAddrs, loop_num: i32) -> async_thrift_tokio::Result<(Box<Vec<i64>>)> {
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

    for _ in 0..loop_num {
        let before = time::now();
        let vec = vec![1;256];
        client.sum_up(vec);
        let end = time::now();

        time_array.push((end - before).num_nanoseconds().unwrap());
    }

    c.close();

    Ok((Box::new(time_array)))
}