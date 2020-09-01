use async_thrift::server;
use async_std::task;
use async_std::io::Error;
use crate::async_thrift_test::tutorial::{CalculatorSyncProcessor, CalculatorSyncHandler};
use async_thrift::transport::async_framed::{TAsyncFramedReadTransportFactory, TAsyncFramedWriteTransportFactory};
use async_thrift::protocol::async_binary::{TAsyncBinaryInputProtocolFactory, TAsyncBinaryOutputProtocolFactory};
use async_std::net::ToSocketAddrs;
use async_trait::async_trait;

pub async fn run_server(addr: impl ToSocketAddrs) {
    let processor = CalculatorSyncProcessor::new(PartHandler {});
    let r_trans_factory = TAsyncFramedReadTransportFactory::new();
    let w_trans_factory = TAsyncFramedWriteTransportFactory::new();
    let i_proto_factory = TAsyncBinaryInputProtocolFactory::new();
    let o_proto_factory = TAsyncBinaryOutputProtocolFactory::new();
    let mut s = server::asynced::TAsyncServer::new(r_trans_factory, i_proto_factory, w_trans_factory, o_proto_factory, processor);

    s.listen(addr).await;
}

struct PartHandler;

#[async_trait]
impl CalculatorSyncHandler for PartHandler {
    async fn handle_add(&self, num1: i32, num2: i32) -> async_thrift::Result<i32> {
        Ok(num1 + num2)
    }
}
