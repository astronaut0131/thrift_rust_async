use async_thrift::server;
use async_std::task;
use async_thrift::transport::async_framed::{TAsyncFramedReadTransportFactory, TAsyncFramedWriteTransportFactory};
use async_thrift::protocol::async_binary::{TAsyncBinaryInputProtocolFactory, TAsyncBinaryOutputProtocolFactory};
use async_std::net::ToSocketAddrs;
use async_trait::async_trait;
use std::collections::BTreeMap;
use crate::async_thrift_test::with_list_map::{ListMapTestSyncHandler,ListMapTestSyncProcessor};


pub async fn run_server(addr: impl ToSocketAddrs) {
    let processor = ListMapTestSyncProcessor::new(PartHandler {});
    let r_trans_factory = TAsyncFramedReadTransportFactory::new();
    let w_trans_factory = TAsyncFramedWriteTransportFactory::new();
    let i_proto_factory = TAsyncBinaryInputProtocolFactory::new();
    let o_proto_factory = TAsyncBinaryOutputProtocolFactory::new();
    let mut s = server::asynced::TAsyncServer::new(r_trans_factory, i_proto_factory, w_trans_factory, o_proto_factory, processor);

    s.listen(addr).await;
}

struct PartHandler;

#[async_trait]
impl ListMapTestSyncHandler for PartHandler {
    async fn handle_sum_up(&self, input: Vec<i32>) -> async_thrift::Result<i32> {
        let mut sum = 0;
        for i in input {
            sum += i;
        }
        return Ok(sum);
    }
    async fn handle_find_value(&self, input: BTreeMap<i32, i32>) -> async_thrift::Result<i32> {
        let ret = input.get(&1).unwrap();
        let x = *ret;
        return Ok(x);
    }
}
