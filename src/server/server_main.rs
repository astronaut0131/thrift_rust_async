pub fn test() {
    println!("hello");
}


use async_std::{
    io::BufReader,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    prelude::*,
    task,
};
use crate::transport::socket::TTcpChannel;
use async_std::io::ErrorKind;
use async_std::io;
use crate::transport::framed::{TFramedReadTransport, TFramedWriteTransport};
use crate::transport::{Read, TReadTransport, TIoChannel};
use crate::protocol::binary::{TBinaryInputProtocol, TBinaryOutputProtocol};
use crate::protocol::{TInputProtocol, TOutputProtocol};
use crate::{ApplicationError, ApplicationErrorKind, TransportErrorKind, new_protocol_error};
use crate::server::TProcessor;
use async_std::sync::Arc;
use log::warn;
use crate::transport::buffered::{TBufferedReadTransport, TBufferedWriteTransport};


pub struct TServer<PRC> where
    PRC: TProcessor + Send + Sync + 'static,
{
    processor: Arc<PRC>,
}

impl<PRC> TServer<PRC> where
    PRC: TProcessor + Send + Sync + 'static,
{
    pub fn new(processor: PRC) -> TServer<PRC> {
        TServer { processor: Arc::new(processor) }
    }

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

            let (read_protocol, write_protocol) = self.new_protocols_for_connection(stream)?;
            task::spawn(handle_incoming_connection_server(
                self.processor.clone(), read_protocol, write_protocol));
        }

        Err(crate::Error::Application(ApplicationError {
            kind: ApplicationErrorKind::Unknown,
            message: "aborted listen loop".into(),
        }))
    }

    fn new_protocols_for_connection(
        &mut self,
        stream: TcpStream,
    ) -> crate::Result<(Box<dyn TInputProtocol + Send>, Box<dyn TOutputProtocol + Send>)> {
        // create the shared tcp stream
        let channel = TTcpChannel::with_stream(stream);

        // split it into two - one to be owned by the
        // input tran/proto and the other by the output
        let (r_chan, w_chan) = channel.split()?;

        // input protocol and transport
        let r_tran = TBufferedReadTransport::new(r_chan);
        let i_port = TBinaryInputProtocol::new(r_tran, true);

        // output protocol and transport
        let w_tran = TBufferedWriteTransport::new(w_chan);
        let o_port = TBinaryOutputProtocol::new(w_tran, true);

        Ok((Box::new(i_port), Box::new(o_port)))
    }
}

// for test transport
async fn handle_incoming_connection(chan: TTcpChannel) {
    let mut t = TFramedReadTransport::new(chan);
    let mut b = vec![0u8; 10];
    let size = t.read(&mut b).await;

    println!("{:?}, {:?}", size, b);
}

// for test protocol
async fn handle_incoming_connection_protocol(chan: TTcpChannel) {
    let mut t = TFramedReadTransport::new(chan);

    let mut protocol = TBinaryInputProtocol::new(t, true);

    let field_identifier = protocol.read_field_begin().await.unwrap();
    let field_contents = protocol.read_string().await.unwrap();
    let field_end = protocol.read_field_end().await.unwrap();

    println!("{:?}, {:?}, {:?}", field_identifier, field_contents, field_end);
}

// real use one
async fn handle_incoming_connection_server<PRC>(
    processor: Arc<PRC>,
    i_prot: Box<dyn TInputProtocol + Send>,
    o_prot: Box<dyn TOutputProtocol + Send>,
) where
    PRC: TProcessor,
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