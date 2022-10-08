pub struct DiscordEvent;

use crate::{models::rpc_event::RPCEvent, utils::create_json};

/// allow you to create JSON payloads to send to the socket for subscribing to events
impl DiscordEvent {
  /// create a json payload for the `SPEAKING_START` event
  /// which will subscribe to the channel supplied
  ///
  /// Arguments:
  /// * `id`: channel id to join
  pub fn speaking_start(id: &str) -> String {
    let mut json = serde_json::json!({
      "cmd": "SUBSCRIBE",
      "evt": RPCEvent::SpeakingStart,
      "args": {
        "channel_id": id
      },
    });

    create_json(&mut json)
  }

  /// create a json payload for the `SPEAKING_STOP` event
  /// which will subscribe to the channel supplied
  ///
  /// Arguments:
  /// * `id`: channel id to join
  pub fn speaking_stop(id: &str) -> String {
    let mut json = serde_json::json!({
      "cmd": "SUBSCRIBE",
      "evt": RPCEvent::SpeakingStop,
      "args": {
        "channel_id": id
      },
    });

    create_json(&mut json)
  }
}