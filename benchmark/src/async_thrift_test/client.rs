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
use crate::async_thrift_test::with_list_map::{ListMapTestSyncClient,TListMapTestSyncClient};
use std::collections::BTreeMap;
use crate::THREAD_NUM;


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

    let mut client = ListMapTestSyncClient::new(i_prot, o_prot);
    let mut sum:i64 = 0;
    let mut r = -1;
    let mut value = -1;
    for i in 0..THREAD_NUM {
        let before = time::now();
        let vec = vec!{1,2,3};
        let mut scores = BTreeMap::new();
        scores.insert(1,100);
        scores.insert(2,200);
        r = client.sum_up(vec).await.unwrap();
        value = client.find_value(scores).await.unwrap();
        let end = time::now();
        time_array.push((end - before).num_nanoseconds().unwrap());
    }

    c.close();

    println!("sum up result {}",r);
    println!("map find value {}",value);
    // println!("done! duration:{:?} ms", (end - start).num_milliseconds());

    // println!("final result {}", sum);
    // println!("Test pass, It's time to cheer!");

    // println!("finish client");
    Ok((Box::new(time_array)))
}

