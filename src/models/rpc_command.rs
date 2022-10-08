use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
  Subscribe,
  Unsubscribe,
  SetUserVoiceSettings,
  SelectVoiceChannel,
  GetSelectedVoiceChannel,
  SelectTextChannel,
  GetVoiceSettings,
  SetVoiceSettings,
}
