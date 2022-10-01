use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Result;

use super::rpc_event::RPCEvent;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "cmd", content = "args")]
pub enum RPCCommand {
  Dispatch,
  Authorize,
  Authenticate,
  GetGuild,
  GetGuilds,
  GetChannel,
  GetChannels,
  CreateChannelInvite,
  GetRelationships,
  GetUser,
  Subscribe(RPCEvent),
  Unsubscribe,
  SetUserVoiceSettings,
  SelectVoiceChannel { channel_id: String },
  GetSelectedVoiceChannel,
  SelectTextChannel,
  GetVoiceSettings,
  SetVoiceSettings,
  CaptureShortcut,
  SetActivity,
  SendActivityJoinInvite,
  CloseActivityJoinReques,
  ActivityInviteUser,
  AcceptActivityInvite,
  InviteBrowser,
  DeepLink,
  ConnectionsCallback,
  BraintreePopupBridgeCallbac,
  GiftCodeBrowser,
  GuildTemplateBrowser,
  Overlay,
  BrowserHandoff,
  SetCertifiedDevices,
  GetImage,
  CreateLobby,
  UpdateLobby,
  DeleteLobby,
  UpdateLobbyMember,
  ConnectToLobby,
  DisconnectFromLobby,
  SendToLobby,
  SearchLobbies,
  ConnectToLobbyVoice,
  DisconnectFromLobbyVoic,
  SetOverlayLocked,
  OpenOverlayActivityInvit,
  OpenOverlayGuildInvite,
  OpenOverlayVoiceSetting,
  ValidateApplication,
  GetEntitlementTicket,
  GetApplicationTicket,
  StartPurchase,
  GetSkus,
  GetEntitlements,
  GetNetworkingConfig,
  NetworkingSystemMetrics,
  NetworkingPeerMetrics,
  NetworkingCreateToken,
  SetUserAchievement,
  GetUserAchievements,
}

impl RPCCommand {
  pub(crate) fn to_json(&self) -> Result<Value> {
    let command_json = match self {
      // Don't know a better way of doing this :/
      Self::Subscribe(event) => {
        let mut event_json = serde_json::to_value(event)?;
        match &mut event_json {
          serde_json::Value::Object(object) => {
            object.insert("cmd".to_string(), "SUBSCRIBE".into());
            object
          }
          _ => panic!("Expected event to be an object"),
        };
        event_json
      }
      _ => serde_json::to_value(self)?,
    };
    println!("{}", serde_json::to_string_pretty(&command_json)?);
    Ok(command_json)
  }
}
