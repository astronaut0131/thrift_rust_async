use super::{TAsyncReadTransport, TAsyncReadTransportFactory, TAsyncWriteTransport, TAsyncWriteTransportFactory};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::cmp;
use async_std::io;
use async_trait::async_trait;
use async_std::net::TcpStream;
use crate::transport::{AsyncRead, AsyncWrite};
use std::io::Cursor;

/// Default capacity of the read buffer in bytes.
const READ_CAPACITY: usize = 4096;

/// Default capacity of the write buffer in bytes.
const WRITE_CAPACITY: usize = 4096;

/// Transport that reads framed messages.
///
/// A `TAsyncFramedReadTransport` maintains a fixed-size internal read buffer.
/// On a call to `TAsyncFramedReadTransport::read(...)` one full message - both
/// fixed-length header and bytes - is read from the wrapped channel and
/// buffered. Subsequent read calls are serviced from the internal buffer
/// until it is exhausted, at which point the next full message is read
/// from the wrapped channel.
///
/// # Examples
///
/// Create and use a `TAsyncFramedReadTransport`.
///
/// ```no_run
/// use std::io::AsyncRead;
/// use thrift::transport::{TAsyncFramedReadTransport, TAsyncTcpChannel};
///
/// let mut c = TAsyncTcpChannel::new();
/// c.open("localhost:9090").unwrap();
///
/// let mut t = TAsyncFramedReadTransport::new(c);
///
/// t.read(&mut vec![0u8; 1]).unwrap();
/// ```
#[derive(Debug)]
pub struct TAsyncFramedReadTransport<C>
    where
        C: AsyncRead,
{
    buf: Vec<u8>,
    pos: usize,
    cap: usize,
    chan: C,
}

impl<C> TAsyncFramedReadTransport<C>
    where
        C: AsyncRead,
{
    /// Create a `TAsyncFramedReadTransport` with a default-sized
    /// internal read buffer that wraps the given `TIoChannel`.
    pub fn new(channel: C) -> TAsyncFramedReadTransport<C> {
        TAsyncFramedReadTransport::with_capacity(READ_CAPACITY, channel)
    }

    /// Create a `TFramedTransport` with an internal read buffer
    /// of size `read_capacity` that wraps the given `TIoChannel`.
    pub fn with_capacity(read_capacity: usize, channel: C) -> TAsyncFramedReadTransport<C> {
        TAsyncFramedReadTransport {
            buf: vec![0; read_capacity], // FIXME: do I actually have to do this?
            pos: 0,
            cap: 0,
            chan: channel,
        }
    }
}

#[async_trait]
impl<C> AsyncRead for TAsyncFramedReadTransport<C>
    where
        C: AsyncRead + std::marker::Send
{
    async fn read(&mut self, b: &mut [u8]) -> io::Result<usize> {
        if self.cap - self.pos == 0 {
            let mut buf = [0; 4];
            self.chan.read(&mut buf).await;
            let mut rdr = Cursor::new(buf);
            let message_size = rdr.read_i32::<BigEndian>().unwrap() as usize;

            let buf_capacity = cmp::max(message_size, READ_CAPACITY);
            self.buf.resize(buf_capacity, 0);

            self.chan.read(&mut self.buf[..message_size]).await;
            self.cap = message_size as usize;
            self.pos = 0;
        }

        let nread = cmp::min(b.len(), self.cap - self.pos);
        b[..nread].clone_from_slice(&self.buf[self.pos..self.pos + nread]);
        self.pos += nread;

        Ok(nread)
    }
}

/// Transport that writes framed messages.
///
/// A `TAsyncFramedWriteTransport` maintains a fixed-size internal write buffer. All
/// writes are made to this buffer and are sent to the wrapped channel only
/// when `TAsyncFramedWriteTransport::flush()` is called. On a flush a fixed-length
/// header with a count of the buffered bytes is written, followed by the bytes
/// themselves.
///
/// # Examples
///
/// Create and use a `TAsyncFramedWriteTransport`.
///
/// ```no_run
/// use std::io::AsyncWrite;
/// use thrift::transport::{TAsyncFramedWriteTransport, TAsyncTcpChannel};
///
/// let mut c = TAsyncTcpChannel::new();
/// c.open("localhost:9090").unwrap();
///
/// let mut t = TAsyncFramedWriteTransport::new(c);
///
/// t.write(&[0x00]).unwrap();
/// t.flush().unwrap();
/// ```
#[derive(Debug)]
pub struct TAsyncFramedWriteTransport<C>
    where
        C: AsyncWrite,
{
    buf: Vec<u8>,
    channel: C,
}

impl<C> TAsyncFramedWriteTransport<C>
    where
        C: AsyncWrite + std::marker::Send
{
    /// Create a `TAsyncFramedWriteTransport` with default-sized internal
    /// write buffer that wraps the given `TIoChannel`.
    pub fn new(channel: C) -> TAsyncFramedWriteTransport<C> {
        TAsyncFramedWriteTransport::with_capacity(WRITE_CAPACITY, channel)
    }

    /// Create a `TAsyncFramedWriteTransport` with an internal write buffer
    /// of size `write_capacity` that wraps the given `TIoChannel`.
    pub fn with_capacity(write_capacity: usize, channel: C) -> TAsyncFramedWriteTransport<C> {
        TAsyncFramedWriteTransport {
            buf: Vec::with_capacity(write_capacity),
            channel,
        }
    }
}

#[async_trait]
impl<C> AsyncWrite for TAsyncFramedWriteTransport<C>
    where
        C: AsyncWrite + std::marker::Send
{
    async fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        let current_capacity = self.buf.capacity();
        let available_space = current_capacity - self.buf.len();
        if b.len() > available_space {
            let additional_space = cmp::max(b.len() - available_space, current_capacity);
            self.buf.reserve(additional_space);
        }

        self.buf.extend_from_slice(b);
        Ok(b.len())
    }

    async fn flush(&mut self) -> io::Result<()> {
        let message_size = self.buf.len();

        if let 0 = message_size {
            return Ok(());
        } else {
            let mut wtr = Vec::new();
            wtr.write_i32::<BigEndian>(message_size as i32).unwrap();

            self.channel.write(&wtr).await;
        }

        // will spin if the underlying channel can't be written to
        let mut byte_index = 0;
        while byte_index < message_size {
            let nwrite = self.channel.write(&self.buf[byte_index..message_size]).await?;
            byte_index = cmp::min(byte_index + nwrite, message_size);
        }

        let buf_capacity = cmp::min(self.buf.capacity(), WRITE_CAPACITY);
        self.buf.resize(buf_capacity, 0);
        self.buf.clear();

        self.channel.flush().await
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
    fn create(&self, channel: Box<dyn AsyncRead + Send>) -> Box<dyn TAsyncReadTransport + Send> {
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
    fn create(&self, channel: Box<dyn AsyncWrite + Send>) -> Box<dyn TAsyncWriteTransport + Send> {
        Box::new(TAsyncFramedWriteTransport::new(channel))
    }
}