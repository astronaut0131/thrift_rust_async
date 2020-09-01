use thrift::protocol::{
    TBinaryInputProtocolFactory, TBinaryOutputProtocolFactory, TCompactInputProtocolFactory,
    TCompactOutputProtocolFactory, TInputProtocolFactory, TOutputProtocolFactory,
};
use thrift::server::TServer;
use thrift::transport::{
    TFramedReadTransportFactory, TFramedWriteTransportFactory, TReadTransportFactory,
    TWriteTransportFactory,
};
use crate::original_thrift_test::with_struct::{CalculatorSyncHandler, Input, Output, CalculatorSyncProcessor};

pub fn run() -> thrift::Result<()> {
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
    fn handle_add(&self, param: Input) -> thrift::Result<Output> {
        thrift::Result::Ok(Output { res: Some(param.num1.unwrap() + param.num2.unwrap()), comment: None })
    }
}