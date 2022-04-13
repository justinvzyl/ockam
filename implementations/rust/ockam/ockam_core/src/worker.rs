use crate::{
    async_trait, compat::boxed::Box, Address, Decodable, LocalMessage, Message, Result, Routed,
};

/// Defines the core interface shared by all Ockam Workers.
///
/// While all methods do not need to be implemented, at the very
/// least, the `Context` and `Message` types need to be specified
/// before a worker can be used in any call to a `Context` API such as
/// `context.start_worker(...)`.
#[async_trait]
pub trait Worker: Send + 'static {
    /// The type of Message the Worker is sent in [`Self::handle_message`].
    type Message: Message;

    /// The API and other resources available for the worker during message
    /// processing.
    ///
    /// Currently, this should be always `ockam::Context` or
    /// `ockam_node::Context` (which are the same type), but in the future
    /// custom node implementations may use a different context type.
    type Context: Send + 'static;

    fn deserialize_message(
        &mut self,
        _address: &Address,
        local_msg: &LocalMessage,
    ) -> Result<Self::Message> {
        let slice = local_msg.transport().payload.as_slice();
        Self::Message::decode(slice).or_else(|_| {
            let mut new_v = serde_bare::to_vec(&serde_bare::Uint(slice.len() as u64))?;

            new_v.append(&mut slice.to_vec());
            Self::Message::decode(&new_v)
        })
    }

    /// Override initialisation behaviour.
    async fn initialize(&mut self, _context: &mut Self::Context) -> Result<()> {
        Ok(())
    }

    /// Override shutdown behaviour.
    async fn shutdown(&mut self, _context: &mut Self::Context) -> Result<()> {
        Ok(())
    }

    /// Try to open and handle a typed message.
    async fn handle_message(
        &mut self,
        _context: &mut Self::Context,
        _msg: Routed<Self::Message>,
    ) -> Result<()> {
        Ok(())
    }
}
