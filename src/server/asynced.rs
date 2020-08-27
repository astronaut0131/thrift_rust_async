use async_std::sync::Arc;
use async_std::net::{ToSocketAddrs,TcpListener,TcpStream};
use async_std::task;
use async_std::io;
use async_std::prelude::*;
use crate::transport::{TAsyncReadTransportFactory,TAsyncWriteTransportFactory};
use crate::protocol::{TAsyncInputProtocolFactory,TAsyncOutputProtocolFactory};

use crate::{ApplicationError, ApplicationErrorKind};

pub struct TAsyncServer<PRC, RTF, IPF, WTF, OPF>
where
    PRC: TAsyncProcessor + 'static,
    RTF: TAsyncReadTransportFactory + 'static,
    IPF: TAsyncInputProtocolFactory + 'static,
    WTF: TAsyncWriteTransportFactory + 'static,
    OPF: TAsyncOutputProtocolFactory + 'static,
{
    r_trans_factory: RTF,
    i_proto_factory: IPF,
    w_trans_factory: WTF,
    o_proto_factory: OPF,
    async_processor: Arc<PRC>,
}

impl<PRC, RTF, IPF, WTF, OPF> TAsyncServer<PRC, RTF, IPF, WTF, OPF>
where
    PRC: TAsyncProcessor + 'static,
    RTF: TAsyncReadTransportFactory + 'static,
    IPF: TAsyncInputProtocolFactory + 'static,
    WTF: TAsyncWriteTransportFactory + 'static,
    OPF: TAsyncOutputProtocolFactory + 'static,
{
    /// Create a `TServer`.
    ///
    /// Each accepted connection has an input and output half, each of which
    /// requires a `TTransport` and `TProtocol`. `TServer` uses
    /// `read_transport_factory` and `input_protocol_factory` to create
    /// implementations for the input, and `write_transport_factory` and
    /// `output_protocol_factory` to create implementations for the output.
    pub fn new(
        read_transport_factory: RTF,
        input_protocol_factory: IPF,
        write_transport_factory: WTF,
        output_protocol_factory: OPF,
        async_processor: PRC,
    ) -> TAsyncServer<PRC, RTF, IPF, WTF, OPF> {
        TAsyncServer {
            r_trans_factory: read_transport_factory,
            i_proto_factory: input_protocol_factory,
            w_trans_factory: write_transport_factory,
            o_proto_factory: output_protocol_factory,
            async_processor: Arc::new(async_processor),
        }
    }
    /// Listen for incoming connections on `listen_address`.
    ///
    /// `listen_address` should implement `ToSocketAddrs` trait.
    ///
    /// Return `()` if successful.
    ///
    /// Return `Err` when the server cannot bind to `listen_address` or there
    /// is an unrecoverable error.
    pub async fn listen<A: ToSocketAddrs>(&mut self, listen_address: A) -> crate::Result<()> {
        let listener = TcpListener::bind(listen_address).await;
        let mut incoming = listener.incoming();
        while let Some(stream) = incoming.next().await {
            match stream {
                Ok(s) => {
                    //println!("Accepting from: {}", stream.peer_addr()?);
                    let (i_prot, o_prot) = self.new_protocols_for_connection(s)?;
                    let processor = self.processor.clone();
                    task::spawn(handle_incoming_connection(processor, i_prot, o_prot));
                }
                Err(e) => {
                    warn!("failed to accept remote connection with error {:?}", e);
                }
            }
        }

        Err(crate::Error::Application(ApplicationError {
            kind: ApplicationErrorKind::Unknown,
            message: "aborted listen loop".into(),
        }))
    }

}

