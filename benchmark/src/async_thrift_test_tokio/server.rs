use async_thrift::server;
use async_std::{
    task,
    net::ToSocketAddrs
};
use std::io::Error;
use async_thrift::transport::async_framed::{TAsyncFramedReadTransportFactory, TAsyncFramedWriteTransportFactory};
use async_thrift::protocol::async_binary::{TAsyncBinaryInputProtocolFactory, TAsyncBinaryOutputProtocolFactory};
use async_trait::async_trait;
use async_thrift::transport::async_buffered::{TAsyncBufferedReadTransportFactory, TAsyncBufferedWriteTransport, TAsyncBufferedWriteTransportFactory};
use crate::async_thrift_test::tutorial::{CalculatorSyncProcessor, CalculatorSyncHandler};

pub async fn run_server(addr: String) {
    let processor = CalculatorSyncProcessor::new(PartHandler {});
    let r_trans_factory = TAsyncBufferedReadTransportFactory::new();
    let w_trans_factory = TAsyncBufferedWriteTransportFactory::new();
    let i_proto_factory = TAsyncBinaryInputProtocolFactory::new();
    let o_proto_factory = TAsyncBinaryOutputProtocolFactory::new();
    let mut s = server::asynced::TAsyncServer::new(r_trans_factory, i_proto_factory, w_trans_factory, o_proto_factory, processor);

    s.listen(addr.as_str()).await;
}

struct PartHandler {
}

#[async_trait]
impl CalculatorSyncHandler for PartHandler {
    async fn handle_ping(&self) -> async_thrift::Result<()> {
        Ok(())
    }
}
