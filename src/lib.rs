pub mod utils;

pub mod errors;
pub mod models;
pub mod opcodes;

mod ipc;
mod ipc_socket;

use serde::{Deserialize, Serialize};

use errors::DiscordRPCError;
pub use ipc::DiscordIpcClient;
use models::{commands::DiscordCommands, events::DiscordEvents};
pub use utils::*;

pub type Result<T, E = DiscordRPCError> = std::result::Result<T, E>;

/// Currently this is used to allow for matching of an event or command type
/// Not all events/commands are implemented so serializing can fail for some of them
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum DiscordMessage {
  Event(Box<DiscordEvents>),
  Command(DiscordCommands),
}
