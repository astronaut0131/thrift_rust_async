use async_thrift::server;
use async_std::task;
use async_thrift::transport::async_framed::{TAsyncFramedReadTransportFactory, TAsyncFramedWriteTransportFactory};
use async_thrift::protocol::async_binary::{TAsyncBinaryInputProtocolFactory, TAsyncBinaryOutputProtocolFactory};
use async_std::net::ToSocketAddrs;
use async_trait::async_trait;
use crate::async_thrift_test::with_struct::{CalculatorServiceSyncProcessor,CalculatorServiceSyncHandler,Number,Material,Operator,Xecption};


pub async fn run_server(addr: impl ToSocketAddrs) {
    let processor = CalculatorServiceSyncProcessor::new(PartHandler {});
    let r_trans_factory = TAsyncFramedReadTransportFactory::new();
    let w_trans_factory = TAsyncFramedWriteTransportFactory::new();
    let i_proto_factory = TAsyncBinaryInputProtocolFactory::new();
    let o_proto_factory = TAsyncBinaryOutputProtocolFactory::new();
    let mut s = server::asynced::TAsyncServer::new(r_trans_factory, i_proto_factory, w_trans_factory, o_proto_factory, processor);

    s.listen(addr).await;
}

struct PartHandler;

#[async_trait]
impl CalculatorServiceSyncHandler for PartHandler {

    async fn handle_calculate(&self, input: Material) -> async_thrift::Result<Number> {
        let a = input.num1.unwrap();
        let b = input.num2.unwrap();
        let op = input.op.unwrap();
        let mut x:i64;
        let mut y:i64;
        match a {
            Number::A(i) => {
                x = i as i64;
            }
            Number::B(i) => {
                x = i;
            }
        }
        match b {
            Number::A(i) => {
                y = i as i64;
            }
            Number::B(i) => {
                y = i;
            }
        }
        match op{
            Operator::Add => {
                return Ok(Number::B(x+y));
            }
            Operator::Divide => {
                return Ok(Number::B(x/y));
            }
        }
    }
}
