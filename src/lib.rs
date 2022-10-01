mod rpc;
mod utils;

pub mod errors;
pub mod opcodes;

use errors::DiscordRPCError;
/// get all stuff from here
pub use utils::*;

// events
// pub use rpc::{Command, Event};

use serde::{Deserialize, Serialize};

pub mod models;
use models::{commands::BasedCommandReturn, events::BasedEvent};

mod ipc;
mod ipc_socket;

pub use ipc::DiscordIpcClient;

pub type Result<T, E = DiscordRPCError> = std::result::Result<T, E>;

/// Currently this is used to allow for matching of an event or type
/// Not all events/commands are implemented so serializing can fail
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum EventReceive {
    Event(BasedEvent),
    CommandReturn(BasedCommandReturn),
}
