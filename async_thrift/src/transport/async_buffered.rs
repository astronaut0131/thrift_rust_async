use std::cmp;
use std::io;

use async_trait::async_trait;

use super::{AsyncRead, AsyncWrite, TAsyncReadTransport, TAsyncReadTransportFactory, TAsyncWriteTransport, TAsyncWriteTransportFactory};

/// Default capacity of the read buffer in bytes.
const READ_CAPACITY: usize = 4096;

/// Default capacity of the write buffer in bytes..
const WRITE_CAPACITY: usize = 4096;


#[derive(Debug)]
pub struct TAsyncBufferedReadTransport<C>
    where
        C: AsyncRead,
{
    buf: Box<[u8]>,
    pos: usize,
    cap: usize,
    chan: C,
}

impl<C> TAsyncBufferedReadTransport<C>
    where
        C: AsyncRead + Send,
{
    /// Create a `TBufferedTransport` with default-sized internal read and
    /// write buffers that wraps the given `TIoChannel`.
    pub fn new(channel: C) -> TAsyncBufferedReadTransport<C> {
        TAsyncBufferedReadTransport::with_capacity(READ_CAPACITY, channel)
    }

    /// Create a `TBufferedTransport` with an internal read buffer of size
    /// `read_capacity` and an internal write buffer of size
    /// `write_capacity` that wraps the given `TIoChannel`.
    pub fn with_capacity(read_capacity: usize, channel: C) -> TAsyncBufferedReadTransport<C> {
        TAsyncBufferedReadTransport {
            buf: vec![0; read_capacity].into_boxed_slice(),
            pos: 0,
            cap: 0,
            chan: channel,
        }
    }

    async fn get_bytes(&mut self) -> io::Result<&[u8]> {
        if self.cap - self.pos == 0 {
            self.pos = 0;
            self.cap = self.chan.read(&mut self.buf).await?;
        }

        Ok(&self.buf[self.pos..self.cap])
    }

    fn consume(&mut self, consumed: usize) {
        // TODO: was a bug here += <-- test somehow
        self.pos = cmp::min(self.cap, self.pos + consumed);
    }
}

#[async_trait]
impl<C> AsyncRead for TAsyncBufferedReadTransport<C>
    where
        C: AsyncRead + Send,
{
    async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut bytes_read = 0;

        loop {
            let nread = {
                let avail_bytes = self.get_bytes().await?;
                let avail_space = buf.len() - bytes_read;
                let nread = cmp::min(avail_space, avail_bytes.len());
                buf[bytes_read..(bytes_read + nread)].copy_from_slice(&avail_bytes[..nread]);
                nread
            };

            self.consume(nread);
            bytes_read += nread;

            if bytes_read == buf.len() || nread == 0 {
                break;
            }
        }

        Ok(bytes_read)
    }
}


#[derive(Debug)]
pub struct TAsyncBufferedWriteTransport<C>
    where
        C: AsyncWrite,
{
    buf: Vec<u8>,
    cap: usize,
    channel: C,
}

impl<C> TAsyncBufferedWriteTransport<C>
    where
        C: AsyncWrite,
{
    /// Create a `TBufferedTransport` with default-sized internal read and
    /// write buffers that wraps the given `TIoChannel`.
    pub fn new(channel: C) -> TAsyncBufferedWriteTransport<C> {
        TAsyncBufferedWriteTransport::with_capacity(WRITE_CAPACITY, channel)
    }

    /// Create a `TBufferedTransport` with an internal read buffer of size
    /// `read_capacity` and an internal write buffer of size
    /// `write_capacity` that wraps the given `TIoChannel`.
    pub fn with_capacity(write_capacity: usize, channel: C) -> TAsyncBufferedWriteTransport<C> {
        assert!(
            write_capacity > 0,
            "write buffer size must be a positive integer"
        );

        TAsyncBufferedWriteTransport {
            buf: Vec::with_capacity(write_capacity),
            cap: write_capacity,
            channel,
        }
    }
}

#[async_trait]
impl<C> AsyncWrite for TAsyncBufferedWriteTransport<C>
    where
        C: AsyncWrite + Send,
{
    async fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if !buf.is_empty() {
            let mut avail_bytes;

            loop {
                avail_bytes = cmp::min(buf.len(), self.cap - self.buf.len());

                if avail_bytes == 0 {
                    self.flush().await?;
                } else {
                    break;
                }
            }

            let avail_bytes = avail_bytes;

            self.buf.extend_from_slice(&buf[..avail_bytes]);
            assert!(self.buf.len() <= self.cap, "copy overflowed buffer");

            Ok(avail_bytes)
        } else {
            Ok(0)
        }
    }

    async fn flush(&mut self) -> io::Result<()> {
        self.channel.write(&self.buf).await?;
        self.channel.flush().await?;
        self.buf.clear();
        Ok(())
    }
}

/// Factory for creating instances of `TAsyncBufferedReadTransport`.
#[derive(Default)]
pub struct TAsyncBufferedReadTransportFactory;

impl TAsyncBufferedReadTransportFactory {
    pub fn new() -> TAsyncBufferedReadTransportFactory {
        TAsyncBufferedReadTransportFactory {}
    }
}

impl TAsyncReadTransportFactory for TAsyncBufferedReadTransportFactory {
    /// Create a `TAsyncBufferedReadTransport`.
    fn create(&self, channel: Box<dyn AsyncRead + Send>) -> Box<dyn TAsyncReadTransport + Send> {
        Box::new(TAsyncBufferedReadTransport::new(channel))
    }
}

/// Factory for creating instances of `TAsyncBufferedWriteTransport`.
#[derive(Default)]
pub struct TAsyncBufferedWriteTransportFactory;

impl TAsyncBufferedWriteTransportFactory {
    pub fn new() -> TAsyncBufferedWriteTransportFactory {
        TAsyncBufferedWriteTransportFactory {}
    }
}

impl TAsyncWriteTransportFactory for TAsyncBufferedWriteTransportFactory {
    /// Create a `TAsyncBufferedWriteTransport`.
    fn create(&self, channel: Box<dyn AsyncWrite + Send>) -> Box<dyn TAsyncWriteTransport + Send> {
        Box::new(TAsyncBufferedWriteTransport::new(channel))
    }
}