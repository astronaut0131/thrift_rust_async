use thrift::protocol::{
    TBinaryInputProtocolFactory, TBinaryOutputProtocolFactory, TCompactInputProtocolFactory,
    TCompactOutputProtocolFactory, TInputProtocolFactory, TOutputProtocolFactory,
};
use thrift::server::TServer;
use thrift::transport::{
    TFramedReadTransportFactory, TFramedWriteTransportFactory, TReadTransportFactory,
    TWriteTransportFactory,
};
use crate::original_thrift_test::tutorial::{CalculatorSyncProcessor, CalculatorSyncHandler};

pub fn run() -> thrift::Result<()>{
    let port = 9090;
    let protocol = "compact";
    let service = "part";
    let listen_address = format!("127.0.0.1:{}", port);

    let r_transport_factory = TFramedReadTransportFactory::new();
    let w_transport_factory = TFramedWriteTransportFactory::new();

    let (i_protocol_factory, o_protocol_factory): (
        Box<TInputProtocolFactory>,
        Box<TOutputProtocolFactory>,
    ) = match &*protocol {
        "binary" => (
            Box::new(TBinaryInputProtocolFactory::new()),
            Box::new(TBinaryOutputProtocolFactory::new()),
        ),
        "compact" => (
            Box::new(TCompactInputProtocolFactory::new()),
            Box::new(TCompactOutputProtocolFactory::new()),
        ),
        unknown => {
            return Err(format!("unsupported transport type {}", unknown).into());
        }
    };

    let processor = CalculatorSyncProcessor::new(PartHandler {});
    let mut server = TServer::new(
        r_transport_factory,
        i_protocol_factory,
        w_transport_factory,
        o_protocol_factory,
        processor,
        1,
    );

    server.listen(listen_address.as_str())
}

struct PartHandler;

impl CalculatorSyncHandler for PartHandler {

    fn handle_add(&self, num1: i32, num2: i32) -> thrift::Result<i32> {
        thrift::Result::Ok(num1 + num2)
    }
}