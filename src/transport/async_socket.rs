use async_std::net::TcpStream;
use async_std::io;
use crate::transport::{AsyncWrite, AsyncRead, AsyncReadHalf, AsyncWriteHalf, TIoChannel};
use async_trait::async_trait;
use async_std::prelude::*;
use async_std::io::ErrorKind;


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
}

#[async_trait]
impl TIoChannel for TAsyncTcpChannel {
    async fn split(self) -> crate::Result<(AsyncReadHalf<Self>, AsyncWriteHalf<Self>)>
        where
            Self: Sized,
    {
        let async_read_half = AsyncReadHalf::new(TAsyncTcpChannel {
            stream: self.stream.clone()
        });
        let async_write_half = AsyncWriteHalf::new(TAsyncTcpChannel {
            stream: self.stream.clone()
        });
        Result::Ok((async_read_half, async_write_half))
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
        //self.if_set(|s| s.write_all().await)
        Ok(())
    }
}

