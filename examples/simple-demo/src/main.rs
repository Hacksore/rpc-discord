use rpc_discord::models::rpc_command::RPCCommand;
use rpc_discord::models::rpc_event::RPCEvent;
use rpc_discord::{DiscordIpcClient, EventReceive};

// get all messages from the client
fn handle_message(event: EventReceive) {
  // get data here
  println!("{:#?}", event);
}

const CHANNEL_ID: &str = "975086424049213564";

#[tokio::main]
async fn main() {
  // load env vars
  dotenv::dotenv().ok();

  // access token from env
  let access_token = dotenv::var("ACCESS_TOKEN").unwrap();

  // client id from env
  let client_id = dotenv::var("CLIENT_ID").unwrap();

  // connect to discord client with overlayed id
  let mut client = DiscordIpcClient::new(&client_id, &access_token)
    .await
    .expect("Client failed to connect");

  // login to the client
  client.login(&access_token).await.unwrap();

  // sub to all events to via this listener
  client.handler(handle_message).await;

  // test join a voice channel
  client
    .emit_command(&RPCCommand::Subscribe(RPCEvent::SpeakingStart {
      channel_id: CHANNEL_ID.to_string(),
    }))
    .await
    .ok();

  client
    .emit_command(&RPCCommand::Subscribe(RPCEvent::SpeakingStop {
      channel_id: CHANNEL_ID.to_string(),
    }))
    .await
    .ok();

  client.emit_command(&RPCCommand::GetSelectedVoiceChannel).await.ok();

  // Keep running after prev thread starts
  loop {
    std::thread::sleep(std::time::Duration::from_millis(1000));
  }

}
