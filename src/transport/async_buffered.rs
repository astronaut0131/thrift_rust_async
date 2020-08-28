// use super::{TAsyncReadTransport, TAsyncReadTransportFactory, TAsyncWriteTransport, TAsyncWriteTransportFactory};
// use async_std::net::TcpStream;
//
// /// Default capacity of the read buffer in bytes.
// const READ_CAPACITY: usize = 4096;
//
// /// Default capacity of the write buffer in bytes..
// const WRITE_CAPACITY: usize = 4096;
//
//
// #[derive(Debug)]
// pub struct TAsyncBufferedReadTransport
// {
//     buf: Box<[u8]>,
//     pos: usize,
//     cap: usize,
//     chan: TcpStream
// }
//
// #[derive(Debug)]
// pub struct TAsyncBufferedWriteTransport
//     where
// {
//     buf: Vec<u8>,
//     cap: usize,
//     channel: TcpStream,
// }
//
// /// Factory for creating instances of `TAsyncBufferedReadTransport`.
// #[derive(Default)]
// pub struct TAsyncBufferedReadTransportFactory;
//
// impl TAsyncBufferedReadTransportFactory {
//     pub fn new() -> TAsyncBufferedReadTransportFactory {
//         TAsyncBufferedReadTransportFactory {}
//     }
// }
//
// impl TAsyncReadTransportFactory for TAsyncBufferedReadTransportFactory {
//     /// Create a `TAsyncBufferedReadTransport`.
//     fn create(&self, channel: TcpStream) -> Box<dyn TAsyncReadTransport+ Send>{
//         Box::new(TAsyncBufferedReadTransport::new(channel))
//     }
// }
//
// /// Factory for creating instances of `TAsyncBufferedWriteTransport`.
// #[derive(Default)]
// pub struct TAsyncBufferedWriteTransportFactory;
//
// impl TAsyncBufferedWriteTransportFactory {
//     pub fn new() -> TAsyncBufferedWriteTransportFactory {
//         TAsyncBufferedWriteTransportFactory {}
//     }
// }
//
// impl TAsyncWriteTransportFactory for TAsyncBufferedWriteTransportFactory {
//     /// Create a `TAsyncBufferedWriteTransport`.
//     fn create(&self, channel: TcpStream) -> Box<dyn TAsyncWriteTransport+ Send> {
//         Box::new(TAsyncBufferedWriteTransport::new(channel))
//     }
// }