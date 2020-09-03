use std::io;
use std::io::ErrorKind;

use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{
    tcp::{OwnedReadHalf, OwnedWriteHalf},
    TcpStream,
};

use crate::transport::{AsyncRead, AsyncReadHalf, AsyncWrite, AsyncWriteHalf, TAsyncIoChannel};

#[derive(Debug, Default)]
pub struct TAsyncTcpChannel {
    stream: Option<TcpStream>,
}

impl TAsyncTcpChannel {
    /// Create a `TAsyncTcpChannel` that wraps an existing `TcpStream`.
    ///
    /// The passed-in stream is assumed to have been opened before being wrapped
    /// by the created `TAsyncTcpChannel` instance.
    pub fn with_stream(stream: TcpStream) -> TAsyncTcpChannel {
        TAsyncTcpChannel {
            stream: Option::Some(stream)
        }
    }

    /// close a tcp channel
    pub fn close(&mut self) {
        // if let Some(ref mut s) = self.stream {
        //     s.shutdown(Shutdown::Both).unwrap();
        // };
    }
}


#[async_trait]
impl AsyncRead for OwnedReadHalf {
    async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        tokio::io::AsyncReadExt::read(self, buf).await
    }
}

#[async_trait]
impl AsyncWrite for OwnedWriteHalf {
    async fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        tokio::io::AsyncWriteExt::write(self, buf).await
    }

    async fn flush(&mut self) -> io::Result<()> {
        tokio::io::AsyncWriteExt::flush(self).await
    }
}

impl TAsyncIoChannel for TAsyncTcpChannel {
    fn split(&mut self) -> crate::Result<(AsyncReadHalf<OwnedReadHalf>, AsyncWriteHalf<OwnedWriteHalf>)>
        where
            Self: Sized,
    {
        let mut channel = self.stream.take();
        let (r_half, w_half) = channel.unwrap().into_split();
        let read_half = AsyncReadHalf::new(r_half);
        let write_half = AsyncWriteHalf::new(w_half);
        Result::Ok((read_half, write_half))
    }
}

#[async_trait]
impl AsyncRead for TAsyncTcpChannel {
    async fn read(&mut self, b: &mut [u8]) -> io::Result<usize> {
        if let Some(ref mut s) = self.stream {
            s.read(b).await
        } else {
            Err(io::Error::new(
                ErrorKind::NotConnected,
                "tcp endpoint not connected",
            ))
        }
    }
}

#[async_trait]
impl AsyncWrite for TAsyncTcpChannel {
    async fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        // println!("in {:?}", b);
        if let Some(ref mut s) = self.stream {
            s.write(b).await
        } else {
            Err(io::Error::new(
                ErrorKind::NotConnected,
                "tcp endpoint not connected",
            ))
        }
    }

    async fn flush(&mut self) -> io::Result<()> {
        if let Some(ref mut s) = self.stream {
            s.flush().await
        } else {
            Err(io::Error::new(
                ErrorKind::NotConnected,
                "tcp endpoint not connected",
            ))
        }
    }
}

