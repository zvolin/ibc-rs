pub mod mock;

use ibc::core::client::context::client_state::ClientState;
use ibc::core::client::context::{ClientExecutionContext, ClientValidationContext};
use ibc::core::client::types::error::ClientError;
use ibc::primitives::proto::Any;

pub trait ClientStateExt<V, E>: ClientState<V, E> + TryFrom<Any, Error = ClientError>
where
    V: ClientValidationContext,
    E: ClientExecutionContext,
{
    fn is_frozen(&self) -> bool;
}
