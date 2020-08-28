use crate::protocol::{TInputProtocol, TOutputProtocol, TMessageIdentifier, TMessageType};
use crate::{ApplicationError, ApplicationErrorKind};

pub mod server_main;
// fixme
// pub mod multiplexed;
use async_trait::async_trait;



/// Handles incoming Thrift messages and dispatches them to the user-defined
/// handler functions.
///
/// An implementation is auto-generated for each Thrift service. When used by a
/// server (for example, a `TSimpleServer`), it will demux incoming service
/// calls and invoke the corresponding user-defined handler function.
///
/// # Examples
///
/// Create and start a server using the auto-generated `TProcessor` for
/// a Thrift service `SimpleService`.
///
/// ```no_run
/// use thrift::protocol::{TInputProtocol, TOutputProtocol};
/// use thrift::server::TProcessor;
///
/// //
/// // auto-generated
/// //
///
/// // processor for `SimpleService`
/// struct SimpleServiceSyncProcessor;
/// impl SimpleServiceSyncProcessor {
///     fn new<H: SimpleServiceSyncHandler>(processor: H) -> SimpleServiceSyncProcessor {
///         unimplemented!();
///     }
/// }
///
/// // `TProcessor` implementation for `SimpleService`
/// impl TProcessor for SimpleServiceSyncProcessor {
///     fn process(&self, i: &mut dyn TInputProtocol, o: &mut dyn TOutputProtocol) -> thrift::Result<()> {
///         unimplemented!();
///     }
/// }
///
/// // service functions for SimpleService
/// trait SimpleServiceSyncHandler {
///     fn service_call(&self) -> thrift::Result<()>;
/// }
///
/// //
/// // user-code follows
/// //
///
/// // define a handler that will be invoked when `service_call` is received
/// struct SimpleServiceHandlerImpl;
/// impl SimpleServiceSyncHandler for SimpleServiceHandlerImpl {
///     fn service_call(&self) -> thrift::Result<()> {
///         unimplemented!();
///     }
/// }
///
/// // instantiate the processor
/// let processor = SimpleServiceSyncProcessor::new(SimpleServiceHandlerImpl {});
///
/// // at this point you can pass the processor to the server
/// // let server = TServer::new(..., processor);
/// ```
#[async_trait]
pub trait TProcessor {
    /// Process a Thrift service call.
    ///
    /// Reads arguments from `i`, executes the user's handler code, and writes
    /// the response to `o`.
    ///
    /// Returns `()` if the handler was executed; `Err` otherwise.
    async fn process(&self, i: &mut (dyn TInputProtocol + Send), o: &mut (dyn TOutputProtocol + Send)) -> crate::Result<()>;
}

/// Convenience function used in generated `TProcessor` implementations to
/// return an `ApplicationError` if thrift message processing failed.
pub async fn handle_process_result(
    msg_ident: &TMessageIdentifier,
    res: crate::Result<()>,
    o_prot: &mut (dyn TOutputProtocol + Send),
) -> crate::Result<()> {
    if let Err(e) = res {
        let e = match e {
            crate::Error::Application(a) => a,
            _ => ApplicationError::new(ApplicationErrorKind::Unknown, format!("{:?}", e)),
        };

        let ident = TMessageIdentifier::new(
            msg_ident.name.clone(),
            TMessageType::Exception,
            msg_ident.sequence_number,
        );

        o_prot.write_message_begin(&ident).await?;
        super::Error::write_application_error_to_out_protocol(&e, o_prot).await?;
        o_prot.write_message_end().await?;
        o_prot.flush().await
    } else {
        Ok(())
    }
}
