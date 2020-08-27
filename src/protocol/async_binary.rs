use crate::transport::{TAsyncReadTransport,TAsyncWriteTransport};
use super::{TAsyncInputProtocolFactory,TAsyncOutputProtocolFactory,TAsyncInputProtocol,TAsyncOutputProtocol};
#[derive(Debug)]
pub struct TAsyncBinaryInputProtocol<T>
    where
        T: TAsyncReadTransport,
{
    strict: bool,
    pub transport: T, // FIXME: shouldn't be public
}

impl<'a, T> TAsyncBinaryInputProtocol<T>
    where
        T: TAsyncReadTransport,
{
    /// Create a `TBinaryInputProtocol` that reads bytes from `transport`.
    ///
    /// Set `strict` to `true` if all incoming messages contain the protocol
    /// version number in the protocol header.
    pub fn new(transport: T, strict: bool) -> TAsyncBinaryInputProtocol<T> {
        TAsyncBinaryInputProtocol {
            strict: strict,
            transport: transport,
        }
    }
}

#[derive(Debug)]
pub struct TAsyncBinaryOutputProtocol<T>
    where
        T: TAsyncWriteTransport,
{
    strict: bool,
    pub transport: T, // FIXME: do not make public; only public for testing!
}


impl<T> TAsyncBinaryOutputProtocol<T>
    where
        T: TAsyncWriteTransport,
{
    /// Create a `TBinaryOutputProtocol` that writes bytes to `transport`.
    ///
    /// Set `strict` to `true` if all outgoing messages should contain the
    /// protocol version number in the protocol header.
    pub fn new(transport: T, strict: bool) -> TAsyncBinaryOutputProtocol<T> {
        TAsyncBinaryOutputProtocol {
            strict: strict,
            transport: transport,
        }
    }
}

/// Factory for creating instances of `TBinaryInputProtocol`.
#[derive(Default)]
pub struct TAsyncBinaryInputProtocolFactory;

impl TAsyncBinaryInputProtocolFactory {
    /// Create a `TBinaryInputProtocolFactory`.
    pub fn new() -> TAsyncBinaryInputProtocolFactory {
        TAsyncBinaryInputProtocolFactory {}
    }
}

impl TAsyncInputProtocolFactory for TAsyncBinaryInputProtocolFactory {
    fn create(&self, transport: Box<dyn TAsyncReadTransport >) -> Box<dyn TAsyncInputProtocol> {
        Box::new(TAsyncBinaryInputProtocol::new(transport, true))
    }
}

/// Factory for creating instances of `TBinaryOutputProtocol`.
#[derive(Default)]
pub struct TAsyncBinaryOutputProtocolFactory;

impl TAsyncBinaryOutputProtocolFactory {
    /// Create a `TBinaryOutputProtocolFactory`.
    pub fn new() -> TAsyncBinaryOutputProtocolFactory {
        TAsyncBinaryOutputProtocolFactory {}
    }
}

impl TAsyncOutputProtocolFactory for TAsyncBinaryOutputProtocolFactory {
    fn create(&self, transport: Box<dyn TAsyncWriteTransport>) -> Box<dyn TAsyncOutputProtocol> {
        Box::new(TAsyncBinaryOutputProtocol::new(transport, true))
    }
}