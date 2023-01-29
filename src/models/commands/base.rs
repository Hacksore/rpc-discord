use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::ChannelData;

/// All command responses that come back from the discord RPC
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "cmd")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CommandReturn {
  GetSelectedVoiceChannel {
    data: Option<ChannelData>,
  },

  /// Get the selected voice channel
  SelectVoiceChannel {
    data: ChannelData,
  },

  /// Subscribe
  Subscribe {
    // TODO: type this
    data: HashMap<String, String>,
  },
  /// Dispatch
  Dispatch {
    // TODO: type this
    data: HashMap<String, String>,
  },
}
