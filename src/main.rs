use rpc_discord::models::rpc_command::RPCCommand;
use rpc_discord::models::rpc_event::RPCEvent;
use rpc_discord::models::{commands::*, events::*};
use rpc_discord::{DiscordIpcClient, EventReceive};

// get all messages from the client
fn handle_message(event: EventReceive) {
    if let EventReceive::CommandReturn(event_type) = event {
        match event_type {
            BasedCommandReturn::GetSelectedVoiceChannel { data } => {
                println!("{:#?}", data);

                if let Some(data) = data {
                    for user in data.voice_states.iter() {
                        println!("{}", user.nick);
                    }
                }
            }
            BasedCommandReturn::SelectVoiceChannel { data } => {
                println!("{:#?}", data.name);
            }
            _ => {
                println!("{:#?}", event_type);
            }
        }
    } else if let EventReceive::Event(event_type) = event {
        match event_type {
            BasedEvent::SpeakingStart { data } => {
                println!("{} started speaking", data.user_id);
            }
            BasedEvent::SpeakingStop { data } => {
                println!("{} stopped speaking", data.user_id);
            }
            _ => {}
        }
    }
}

const CHANNEL_ID: &str = "1006463274184867870";

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

    // test
    client.login(&access_token).await.unwrap();

    // sub to all events to via this listener
    client.handler(handle_message).await;

    client
        .emit(&RPCCommand::Subscribe(RPCEvent::SpeakingStart {
            channel_id: CHANNEL_ID.to_string(),
        }))
        .await
        .ok();

    client
        .emit(&RPCCommand::Subscribe(RPCEvent::SpeakingStop {
            channel_id: CHANNEL_ID.to_string(),
        }))
        .await
        .ok();

    client.emit(&RPCCommand::GetSelectedVoiceChannel).await.ok();

    // Keep running after prev thread starts
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    // Uncomment if removing the loop above
    // Ok(())
}
