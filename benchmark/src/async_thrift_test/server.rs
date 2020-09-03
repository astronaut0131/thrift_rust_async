use async_std::{net::ToSocketAddrs, task, io};
use async_trait::async_trait;

use async_thrift::protocol::async_binary::{TAsyncBinaryInputProtocolFactory, TAsyncBinaryOutputProtocolFactory};
use async_thrift::server;
use async_thrift::transport::async_buffered::{TAsyncBufferedReadTransportFactory, TAsyncBufferedWriteTransport, TAsyncBufferedWriteTransportFactory};
use async_thrift::transport::async_framed::{TAsyncFramedReadTransportFactory, TAsyncFramedWriteTransportFactory};

use crate::async_thrift_test::tutorial::{CalculatorSyncHandler, CalculatorSyncProcessor};
use crate::async_thrift_test::echo::{LongMessageTestSyncProcessor, LongMessageTestSyncHandler};

pub async fn run_server(addr: String) {
    let processor = LongMessageTestSyncProcessor::new(PartHandler {});
    let r_trans_factory = TAsyncBufferedReadTransportFactory::new();
    let w_trans_factory = TAsyncBufferedWriteTransportFactory::new();
    let i_proto_factory = TAsyncBinaryInputProtocolFactory::new();
    let o_proto_factory = TAsyncBinaryOutputProtocolFactory::new();
    let mut s = server::asynced::TAsyncServer::new(r_trans_factory, i_proto_factory, w_trans_factory, o_proto_factory, processor);

    s.listen(addr.as_str()).await;
}

struct PartHandler {}

#[async_trait]
// impl CalculatorSyncHandler for PartHandler {
//     async fn handle_ping(&self) -> async_thrift::Result<()> {
//         Ok(())
//     }
// }
impl LongMessageTestSyncHandler for PartHandler {
    // async fn handle_ping(&self) -> async_thrift::Result<()> {
    //     Ok(())
    // }

    async fn handle_echo(&self, input: Vec<i8>) -> async_thrift::Result<Vec<i8>> {
        Ok(input)
    }
}
