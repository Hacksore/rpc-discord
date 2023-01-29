use rpc_discord::models::commands::CommandReturn;
use rpc_discord::models::events::EventReturn;
use rpc_discord::models::rpc_event::RPCEvent;
use rpc_discord::models::rpc_command::RPCCommand;
use rpc_discord::{DiscordIpcClient, EventReceive};

// get all messages from the client
fn handle_message(event: EventReceive) {
  if let EventReceive::Command(event_type) = event {
    match event_type {
      CommandReturn::GetSelectedVoiceChannel { data } => {
        println!("{:#?}", data);

        if let Some(data) = data {
          for user in data.voice_states.iter() {
            println!("{}", user.nick);
          }
        }
      }
      CommandReturn::SelectVoiceChannel { data } => {
        println!("{:#?}", data.name);
      }
      _ => {
        println!("{:#?}", event_type);
      }
    }
  } else if let EventReceive::Event(event_type) = event {
    match event_type.as_ref() {
      EventReturn::SpeakingStart { data } => {
        println!("{} started speaking", data.user_id);
      }
      EventReturn::SpeakingStop { data } => {
        println!("{} stopped speaking", data.user_id);
      }
      _ => {}
    }
  }
}

const CHANNEL_ID: &str = "513983880303345665";

#[tokio::main]
async fn main() -> rpc_discord::Result<()> {
  // load env vars
  dotenv::dotenv().ok();

  // access token from env
  let access_token = dotenv::var("ACCESS_TOKEN").expect("You must set an ACCESS_TOKEN");

  // client id from env
  let client_id = dotenv::var("CLIENT_ID").expect("You must set CLIENT_ID");

  // connect to discord client with overlayed id
  let mut rpc = DiscordIpcClient::new(&client_id, &access_token)
    .await
    .expect("Client failed to connect");

  // ask discord for the current channel
  rpc
    .emit_command(&RPCCommand::GetSelectedVoiceChannel)
    .await
    .ok();

  rpc
    .emit_command(&RPCCommand::Subscribe(RPCEvent::SpeakingStart {
      channel_id: CHANNEL_ID.to_string(),
    }))
    .await
    .ok();

  rpc
    .emit_command(&RPCCommand::Subscribe(RPCEvent::SpeakingStop {
      channel_id: CHANNEL_ID.to_string(),
    }))
    .await
    .ok();

  // create a handler to listen to all events/messages
  // TODO: this has no access to the client ref :(
  rpc.handler(handle_message).await;

  // Keep running after prev thread starts
  loop {
    std::thread::sleep(std::time::Duration::from_millis(1000));
  }
}
