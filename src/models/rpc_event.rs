use serde::{Deserialize, Serialize};

// TODO: move this to somewhere else
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "evt", content = "args")]
pub enum RPCEvent {
  CurrentUserUpdate,
  VoiceChannelSelect,
  VoiceStateCreate,
  VoiceStateDelete,
  VoiceStateUpdate,
  VoiceSettingsUpdate,
  VoiceConnectionStatus,
  SpeakingStart { channel_id: String },
  SpeakingStop { channel_id: String },
  Ready,
  Error,
}
