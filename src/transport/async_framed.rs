use super::{TAsyncReadTransport, TAsyncReadTransportFactory, TAsyncWriteTransport, TAsyncWriteTransportFactory};
use futures::AsyncRead;
use async_std::net::TcpStream;

/// Default capacity of the read buffer in bytes.
const READ_CAPACITY: usize = 4096;

/// Default capacity of the write buffer in bytes.
const WRITE_CAPACITY: usize = 4096;

#[derive(Debug)]
pub struct TAsyncFramedReadTransport
{
    buf: Vec<u8>,
    pos: usize,
    cap: usize,
    chan: TcpStream
}

impl TAsyncFramedReadTransport
{
    /// Create a `TFramedReadTransport` with a default-sized
    /// internal read buffer that wraps the given `TIoChannel`.
    pub fn new(channel: TcpStream) -> TAsyncFramedReadTransport {
        TAsyncFramedReadTransport::with_capacity(READ_CAPACITY, channel)
    }

    /// Create a `TFramedTransport` with an internal read buffer
    /// of size `read_capacity` that wraps the given `TIoChannel`.
    pub fn with_capacity(read_capacity: usize, channel: TcpStream) -> TAsyncFramedReadTransport {
        TAsyncFramedReadTransport {
            buf: vec![0; read_capacity], // FIXME: do I actually have to do this?
            pos: 0,
            cap: 0,
            chan: channel,
        }
    }

}

#[derive(Debug)]
pub struct TAsyncFramedWriteTransport

{
    buf: Vec<u8>,
    channel: TcpStream
}

impl TAsyncFramedWriteTransport

{
    /// Create a `TFramedWriteTransport` with default-sized internal
    /// write buffer that wraps the given `TIoChannel`.
    pub fn new(channel: TcpStream) -> TAsyncFramedWriteTransport{
        TAsyncFramedWriteTransport::with_capacity(WRITE_CAPACITY, channel)
    }

    /// Create a `TFramedWriteTransport` with an internal write buffer
    /// of size `write_capacity` that wraps the given `TIoChannel`.
    pub fn with_capacity(write_capacity: usize, channel: TcpStream) -> TAsyncFramedWriteTransport {
        TAsyncFramedWriteTransport {
            buf: Vec::with_capacity(write_capacity),
            channel,
        }
    }
}

/// Factory for creating instances of `TAsyncFramedReadTransport`.
#[derive(Default)]
pub struct TAsyncFramedReadTransportFactory;

impl TAsyncFramedReadTransportFactory {
    pub fn new() -> TAsyncFramedReadTransportFactory {
        TAsyncFramedReadTransportFactory {}
    }
}

impl TAsyncReadTransportFactory for TAsyncFramedReadTransportFactory {
    /// Create a `TAsyncFramedReadTransport`.
    fn create(&self, channel: TcpStream) -> Box<dyn TAsyncReadTransport> {
        Box::new(TAsyncFramedReadTransport::new(channel))
    }
}
/// Factory for creating instances of `TAsyncFramedWriteTransport`.
#[derive(Default)]
pub struct TAsyncFramedWriteTransportFactory;

impl TAsyncFramedWriteTransportFactory {
    pub fn new() -> TAsyncFramedWriteTransportFactory {
        TAsyncFramedWriteTransportFactory {}
    }
}

impl TAsyncWriteTransportFactory for TAsyncFramedWriteTransportFactory {
    /// Create a `TAsyncFramedWriteTransport`.
    fn create(&self, channel: TcpStream) -> Box<dyn TAsyncWriteTransport> {
        Box::new(TAsyncFramedWriteTransport::new(channel))
    }
}