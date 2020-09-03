use thrift::protocol::{
    TBinaryInputProtocolFactory, TBinaryOutputProtocolFactory, TCompactInputProtocolFactory,
    TCompactOutputProtocolFactory, TInputProtocolFactory, TOutputProtocolFactory,
};
use thrift::server::TServer;
use thrift::transport::{TBufferedReadTransportFactory, TBufferedWriteTransportFactory, TFramedReadTransportFactory, TFramedWriteTransportFactory, TReadTransportFactory, TWriteTransportFactory};

use crate::sync_thrift_test::tutorial::{CalculatorSyncHandler, CalculatorSyncProcessor};

pub fn run(addr: &str) -> thrift::Result<()> {
    let port = 9090;
    let protocol = "binary";
    let service = "part";
    let listen_address = addr;

    let r_transport_factory = TBufferedReadTransportFactory::new();
    let w_transport_factory = TBufferedWriteTransportFactory::new();

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

    server.listen(listen_address)
}

struct PartHandler;

impl CalculatorSyncHandler for PartHandler {
    fn handle_ping(&self) -> thrift::Result<()> {
        Ok(())
    }
}