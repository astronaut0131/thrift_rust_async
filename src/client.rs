use async_std::io::stdin;
use async_std::{
    io::BufReader,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    prelude::*,
    task,
};
use async_std::io::Error;

pub type Result<T> = std::result::Result<T, Error>;


use futures::{select, FutureExt};
use rs_thrift::transport::socket::TTcpChannel;
use rs_thrift::transport::framed::TFramedWriteTransport;
use rs_thrift::transport::Write;

// a client for chat room, should run in another process (not tread)
pub async fn try_run(addr: impl ToSocketAddrs) -> Result<()> {
    let stream = TcpStream::connect(addr).await?;
    let mut c = TTcpChannel::with_stream(stream);
    let mut t = TFramedWriteTransport::new(c);

    t.write(&[0x00, 0x00, 0x00, 0x02, 0x02, 0x01]).await;
    t.flush().await;
    Ok(())
}