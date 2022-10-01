mod rpc;
mod utils;

pub mod errors;
pub mod opcodes;
pub mod models;

mod ipc;
mod ipc_socket;

use serde::{Deserialize, Serialize};

pub use ipc::DiscordIpcClient;
pub use utils::*;
use models::{commands::BasedCommandReturn, events::BasedEvent};
use errors::DiscordRPCError;

pub type Result<T, E = DiscordRPCError> = std::result::Result<T, E>;

/// Currently this is used to allow for matching of an event or type
/// Not all events/commands are implemented so serializing can fail
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum EventReceive {
  Event(BasedEvent),
  CommandReturn(BasedCommandReturn),
}
