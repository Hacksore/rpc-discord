pub mod utils;
pub mod rpc;
pub mod errors;
pub mod models;
pub mod opcodes;

mod ipc;
mod ipc_socket;

use serde::{Deserialize, Serialize};

use errors::DiscordRPCError;
pub use ipc::DiscordIpcClient;
use models::{commands::BasedCommandReturn, events::BasedEvent};
pub use utils::*;

pub use rpc::discord_command::DiscordCommand;

pub type Result<T, E = DiscordRPCError> = std::result::Result<T, E>;

/// Currently this is used to allow for matching of an event or type
/// Not all events/commands are implemented so serializing can fail
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum EventReceive {
  Event(BasedEvent),
  CommandReturn(BasedCommandReturn),
}
