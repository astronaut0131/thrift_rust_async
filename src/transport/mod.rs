pub mod socket;
mod mem;
pub mod framed;

use async_std::io;
use async_trait::async_trait;

// Identifies a transport used by a `TInputProtocol` to receive bytes.
#[async_trait]
pub trait TReadTransport: Read {}

/// Helper type used by a server to create `TReadTransport` instances for
/// accepted client connections.
#[async_trait]
pub trait TReadTransportFactory {
    /// Create a `TTransport` that wraps a channel over which bytes are to be read.
    fn create(&self, channel: Box<dyn Read + Send>) -> Box<dyn TReadTransport + Send>;
}

/// Identifies a transport used by `TOutputProtocol` to send bytes.
#[async_trait]
pub trait TWriteTransport: Write {}

/// Helper type used by a server to create `TWriteTransport` instances for
/// accepted client connections.
#[async_trait]
pub trait TWriteTransportFactory {
    /// Create a `TTransport` that wraps a channel over which bytes are to be sent.
    fn create(&self, channel: Box<dyn Write + Send>) -> Box<dyn TWriteTransport + Send>;
}

#[async_trait]
pub trait Read {
    async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
}

#[async_trait]
pub trait Write {
    async fn write(&mut self, buf: &[u8]) -> io::Result<usize>;

    async fn flush(&mut self) -> io::Result<()>;
}

use std::ops::{Deref, DerefMut};


#[async_trait]
pub trait TIoChannel: Read + Write {
    /// Split the channel into a readable half and a writable half, where the
    /// readable half implements `io::Read` and the writable half implements
    /// `io::Write`. Returns `None` if the channel was not initialized, or if it
    /// cannot be split safely.
    ///
    /// Returned halves may share the underlying OS channel or buffer resources.
    /// Implementations **should ensure** that these two halves can be safely
    /// used independently by concurrent threads.
    async fn split(self) -> crate::Result<(crate::transport::ReadHalf<Self>, crate::transport::WriteHalf<Self>)>
        where
            Self: Sized;
}

// The readable half of an object returned from `TIoChannel::split`.
#[derive(Debug)]
pub struct ReadHalf<C>
    where
        C: Read,
{
    handle: C,
}

/// The writable half of an object returned from `TIoChannel::split`.
#[derive(Debug)]
pub struct WriteHalf<C>
    where
        C: Write,
{
    handle: C,
}

impl<C> ReadHalf<C>
    where
        C: Read,
{
    /// Create a `ReadHalf` associated with readable `handle`
    pub fn new(handle: C) -> ReadHalf<C> {
        ReadHalf { handle }
    }
}

impl<C> WriteHalf<C>
    where
        C: Write,
{
    /// Create a `WriteHalf` associated with writable `handle`
    pub fn new(handle: C) -> WriteHalf<C> {
        WriteHalf { handle }
    }
}

#[async_trait]
impl<C> Read for ReadHalf<C>
    where
        C: Read + std::marker::Send,
{
    async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.handle.read(buf).await
    }
}

#[async_trait]
impl<C> Write for WriteHalf<C>
    where
        C: Write + std::marker::Send,
{
    async fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.handle.write(buf).await
    }

    async fn flush(&mut self) -> io::Result<()> {
        self.handle.flush().await
    }
}

impl<C> Deref for ReadHalf<C>
    where
        C: Read,
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl<C> DerefMut for ReadHalf<C>
    where
        C: Read,
{
    fn deref_mut(&mut self) -> &mut C {
        &mut self.handle
    }
}

impl<C> Deref for WriteHalf<C>
    where
        C: Write,
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl<C> DerefMut for WriteHalf<C>
    where
        C: Write,
{
    fn deref_mut(&mut self) -> &mut C {
        &mut self.handle
    }
}


