use async_std::net::TcpStream;

mod async_buffered;
mod async_framed;

/// Empty traits just for abstraction, cauz we have to use async functions
/// Identifies a transport used by a `TAsyncInputProtocol` to receive bytes.
pub trait TAsyncReadTransport{}
/// Identifies a transport used by `TAsyncOutputProtocol` to send bytes.
pub trait TAsyncWriteTransport{}

/// Helper type used by a server to create `TAsyncReadTransport` instances for
/// accepted client connections.
pub trait TAsyncReadTransportFactory {
    /// Create a `TTransport` that wraps a channel over which bytes are to be read.
    fn create(&self, channel: TcpStream) -> Box<dyn TAsyncReadTransport>;
}

/// Helper type used by a server to create `TWriteTransport` instances for
/// accepted client connections.
pub trait TAsyncWriteTransportFactory {
    /// Create a `TTransport` that wraps a channel over which bytes are to be sent.
    fn create(&self, channel: TcpStream) -> Box<dyn TAsyncWriteTransport>;
}

impl<T> TAsyncReadTransportFactory for Box<T>
    where
        T: TAsyncReadTransportFactory,
{
    fn create(&self, channel: TcpStream) -> Box<dyn TAsyncReadTransport> {
        (**self).create(channel)
    }
}

impl<T> TAsyncWriteTransportFactory for Box<T>
    where
        T: TAsyncWriteTransportFactory,
{
    fn create(&self, channel: TcpStream) -> Box<dyn TAsyncWriteTransport> {
        (**self).create(channel)
    }
}
