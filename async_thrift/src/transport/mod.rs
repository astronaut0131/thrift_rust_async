use async_std::net::TcpStream;
use async_std::io;
use async_trait::async_trait;
use std::ops::{Deref, DerefMut};

pub mod async_buffered;
pub mod async_framed;
pub mod async_socket;
use futures::AsyncWriteExt;

#[async_trait]
pub trait AsyncRead {
    async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
}

#[async_trait]
pub trait AsyncWrite {
    async fn write(&mut self, buf: &[u8]) -> io::Result<usize>;

    async fn flush(&mut self) -> io::Result<()>;
}

#[async_trait]
impl<R: AsyncRead + ?Sized + Send> AsyncRead for Box<R> {
    async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (**self).read(buf).await
    }
}

#[async_trait]
impl<R: AsyncWrite + ?Sized + Send> AsyncWrite for Box<R> {
    async fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (**self).write(buf).await
    }
    async fn flush(&mut self) -> io::Result<()> {
        (**self).flush().await
    }
}

/// Identifies a transport used by a `TAsyncInputProtocol` to receive bytes.
#[async_trait]
pub trait TAsyncReadTransport: AsyncRead {}

/// Identifies a transport used by `TAsyncOutputProtocol` to send bytes.
#[async_trait]
pub trait TAsyncWriteTransport: AsyncWrite {}

impl<T> TAsyncReadTransport for T where T: AsyncRead {}

impl<T> TAsyncWriteTransport for T where T: AsyncWrite {}

/// Helper type used by a server to create `TAsyncReadTransport` instances for
/// accepted client connections.
pub trait TAsyncReadTransportFactory {
    /// Create a `TTransport` that wraps a channel over which bytes are to be read.
    fn create(&self, channel: Box<dyn AsyncRead + Send>) -> Box<dyn TAsyncReadTransport + Send>;
}

/// Helper type used by a server to create `TWriteTransport` instances for
/// accepted client connections.
pub trait TAsyncWriteTransportFactory {
    /// Create a `TTransport` that wraps a channel over which bytes are to be sent.
    fn create(&self, channel: Box<dyn AsyncWrite + Send>) -> Box<dyn TAsyncWriteTransport + Send>;
}

impl<T> TAsyncReadTransportFactory for Box<T>
    where
        T: TAsyncReadTransportFactory,
{
    fn create(&self, channel: Box<dyn AsyncRead + Send>) -> Box<dyn TAsyncReadTransport + Send> {
        (**self).create(channel)
    }
}

impl<T> TAsyncWriteTransportFactory for Box<T>
    where
        T: TAsyncWriteTransportFactory,
{
    fn create(&self, channel: Box<dyn AsyncWrite + Send>) -> Box<dyn TAsyncWriteTransport + Send> {
        (**self).create(channel)
    }
}


pub trait TAsyncIoChannel: AsyncRead + AsyncWrite {
    /// Split the channel into a readable half and a writable half, where the
    /// readable half implements `io::AsyncRead` and the writable half implements
    /// `io::AsyncWrite`. Returns `None` if the channel was not initialized, or if it
    /// cannot be split safely.
    ///
    /// Returned halves may share the underlying OS channel or buffer resources.
    /// Implementations **should ensure** that these two halves can be safely
    /// used independently by concurrent threads.
    fn split(&self) -> crate::Result<(AsyncReadHalf<Self>, AsyncWriteHalf<Self>)>
        where
            Self: Sized;
}

// The readable half of an object returned from `TIoChannel::split`.
#[derive(Debug)]
pub struct AsyncReadHalf<C>
    where
        C: AsyncRead,
{
    handle: C,
}

/// The writable half of an object returned from `TIoChannel::split`.
#[derive(Debug)]
pub struct AsyncWriteHalf<C>
    where
        C: AsyncWrite,
{
    handle: C,
}

impl<C> AsyncReadHalf<C>
    where
        C: AsyncRead,
{
    /// Create a `AsyncReadHalf` associated with readable `handle`
    pub fn new(handle: C) -> AsyncReadHalf<C> {
        AsyncReadHalf { handle }
    }
}

impl<C> AsyncWriteHalf<C>
    where
        C: AsyncWrite,
{
    /// Create a `AsyncWriteHalf` associated with writable `handle`
    pub fn new(handle: C) -> AsyncWriteHalf<C> {
        AsyncWriteHalf { handle }
    }
}

#[async_trait]
impl<C> AsyncRead for AsyncReadHalf<C>
    where
        C: AsyncRead + std::marker::Send,
{
    async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.handle.read(buf).await
    }
}

#[async_trait]
impl<C> AsyncWrite for AsyncWriteHalf<C>
    where
        C: AsyncWrite + std::marker::Send,
{
    async fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.handle.write(buf).await
    }

    async fn flush(&mut self) -> io::Result<()> {
        self.handle.flush().await
    }
}

impl<C> Deref for AsyncReadHalf<C>
    where
        C: AsyncRead,
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl<C> DerefMut for AsyncReadHalf<C>
    where
        C: AsyncRead,
{
    fn deref_mut(&mut self) -> &mut C {
        &mut self.handle
    }
}

impl<C> Deref for AsyncWriteHalf<C>
    where
        C: AsyncWrite,
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl<C> DerefMut for AsyncWriteHalf<C>
    where
        C: AsyncWrite,
{
    fn deref_mut(&mut self) -> &mut C {
        &mut self.handle
    }
}