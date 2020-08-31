use async_std::sync::Arc;
use async_std::net::{ToSocketAddrs, TcpListener, TcpStream};
use async_std::task;
use async_std::io;
use async_std::prelude::*;
use crate::transport::{TAsyncReadTransportFactory, TAsyncWriteTransportFactory, TAsyncReadTransport, TAsyncWriteTransport};
use crate::transport::async_socket::{TAsyncTcpChannel};
use crate::protocol::{TAsyncInputProtocolFactory, TAsyncOutputProtocolFactory, TAsyncInputProtocol, TAsyncOutputProtocol};
use crate::errors::TransportErrorKind;
use crate::{ApplicationError, ApplicationErrorKind};
use super::TAsyncProcessor;
use crate::transport::TAsyncIoChannel;

pub struct TAsyncServer<PRC, RTF, IPF, WTF, OPF>
    where
        PRC: TAsyncProcessor + Send + Sync + 'static,
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
        PRC: TAsyncProcessor + Send + Sync + 'static,
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
        println!("into listen");
        let listener = TcpListener::bind(listen_address).await?;

        let mut incoming = listener.incoming();
        println!("begin listening to:");
        while let Some(stream) = incoming.next().await {
            // stream is a new tcp connection stream
            let stream = stream?;
            println!("Accepting from: {}", stream.peer_addr()?);
            // new tcp reader thread

            let (read_protocol, write_protocol) = self.new_protocols_for_connection(stream).await?;
            task::spawn(handle_incoming_connection_server(
                self.async_processor.clone(), read_protocol, write_protocol));
        }

        Err(crate::Error::Application(ApplicationError {
            kind: ApplicationErrorKind::Unknown,
            message: "aborted listen loop".into(),
        }))
    }

    /// build io channel for connection
    /// return input channel and output channel
    async fn new_protocols_for_connection(
        &mut self,
        stream: TcpStream,
    ) -> crate::Result<(Box<dyn TAsyncInputProtocol + Send>, Box<dyn TAsyncOutputProtocol + Send>)> {
        // create the shared tcp stream
        let channel = TAsyncTcpChannel::with_stream(stream);

        // split it into two - one to be owned by the
        // input tran/proto and the other by the output
        let (r_chan, w_chan) = channel.split()?;

        // input protocol and transport
        let r_tran = self.r_trans_factory.create(Box::new(r_chan));
        let i_prot = self.i_proto_factory.create(r_tran);

        // output protocol and transport
        let w_tran = self.w_trans_factory.create(Box::new(w_chan));
        let o_prot = self.o_proto_factory.create(w_tran);

        Ok((i_prot, o_prot))
    }
}


/// handle one connection using processor
async fn handle_incoming_connection_server<PRC>(
    processor: Arc<PRC>,
    i_prot: Box<dyn TAsyncInputProtocol + Send>,
    o_prot: Box<dyn TAsyncOutputProtocol + Send>,
) where
    PRC: TAsyncProcessor,
{
    let mut i_prot = i_prot;
    let mut o_prot = o_prot;
    loop {
        match processor.process(&mut *i_prot, &mut *o_prot).await {
            Ok(()) => {}
            Err(err) => {
                match err {
                    crate::Error::Transport(ref transport_err) if transport_err.kind == TransportErrorKind::EndOfFile => {}
                    other => warn!("processor completed with error: {:?}", other),
                }
                break;
            }
        }
    }
}