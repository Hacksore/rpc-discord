use rpc_discord::{DiscordCommand, DiscordIpcClient, EventReceive, rpc::discord_event::DiscordEvent};

// get all messages from the client
fn handle_message(event: EventReceive) {
  println!("event {:#?}", event);
}

const CHANNEL_ID: &str = "1023308619497873419";

#[tokio::main]
async fn main() -> rpc_discord::Result<()> {
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


  client.login().await.ok();

  // sub to all events to via this listener
  client.handler(handle_message).await;

  // // get voice channel
  // println!("Sending get selected payload...");
  client.emit_string(DiscordCommand::select_voice_channel("975086424049213564")).await.ok();

  client
    .emit_string(DiscordEvent::speaking_start(CHANNEL_ID))
    .await
    .ok();

  client
    .emit_string(DiscordEvent::speaking_stop(CHANNEL_ID))
    .await
    .ok();

  // Keep running after prev thread starts
  loop {
    std::thread::sleep(std::time::Duration::from_millis(1000));
  }

  // Uncomment if removing the loop above
  // Ok(())
}
