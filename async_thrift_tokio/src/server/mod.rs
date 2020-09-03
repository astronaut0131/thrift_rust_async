use async_trait::async_trait;

use crate::errors::{ApplicationError, ApplicationErrorKind};
use crate::protocol::{TAsyncInputProtocol, TAsyncOutputProtocol};
use crate::protocol::{TMessageIdentifier, TMessageType};

pub mod asynced;

#[async_trait]
pub trait TAsyncProcessor {
    /// Process a Thrift service call.
    ///
    /// Reads arguments from `i`, executes the user's handler code, and writes
    /// the response to `o`.
    ///
    /// Returns `()` if the handler was executed; `Err` otherwise.
    async fn process(&self, i: &mut (dyn TAsyncInputProtocol + Send), o: &mut (dyn TAsyncOutputProtocol + Send)) -> crate::Result<()>;
}

/// Convenience function used in generated `TProcessor` implementations to
/// return an `ApplicationError` if thrift message processing failed.
pub async fn handle_process_result(
    msg_ident: &TMessageIdentifier,
    res: crate::Result<()>,
    o_prot: &mut (dyn TAsyncOutputProtocol + Send),
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
