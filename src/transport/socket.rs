use async_std::net::TcpStream;
use async_std::io;
use crate::transport::{Write, Read, ReadHalf, WriteHalf, TIoChannel};
use async_trait::async_trait;
use async_std::prelude::*;
use async_std::io::ErrorKind;


#[derive(Debug, Default)]
pub struct TTcpChannel {
    stream: Option<TcpStream>,
}

impl TTcpChannel {
    /// Create a `TTcpChannel` that wraps an existing `TcpStream`.
    ///
    /// The passed-in stream is assumed to have been opened before being wrapped
    /// by the created `TTcpChannel` instance.
    pub fn with_stream(stream: TcpStream) -> TTcpChannel {
        TTcpChannel {
            stream: Option::Some(stream)
        }
    }
}

impl TIoChannel for TTcpChannel {
    fn split(self) -> crate::Result<(ReadHalf<Self>, WriteHalf<Self>)>
        where
            Self: Sized,
    {
        let read_half = ReadHalf::new(TTcpChannel {
            stream: self.stream.clone()
        });
        let write_half = WriteHalf::new(TTcpChannel {
            stream: self.stream.clone()
        });
        Result::Ok((read_half, write_half))
    }
}


#[async_trait]
impl Read for TTcpChannel {
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
impl Write for TTcpChannel {
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

