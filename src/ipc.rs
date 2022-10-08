use crate::ipc_socket::DiscordIpcSocket;
use crate::models::events::BasedEvent;
use crate::models::rpc_command::RPCCommand;
use crate::models::shared::User;
use crate::opcodes::OPCODES;

use crate::EventReceive;
use crate::Result;
use serde_json::json;
use uuid::Uuid;

type Callback = fn();

#[allow(dead_code)]
#[allow(missing_docs)]
/// A wrapper struct for the functionality contained in the
/// underlying [`DiscordIpc`](trait@DiscordIpc) trait.
pub struct DiscordIpcClient {
  /// Client ID of the IPC client.
  pub client_id: String,
  pub access_token: String,
  // Socket ref to the open socket
  socket: DiscordIpcSocket,

  /// User data object
  pub userdata: Option<User>,

  /// ready callback?
  pub ready_callback: Option<Callback>
}

impl DiscordIpcClient {
  /// Creates a new `DiscordIpcClient`.
  ///
  /// # Examples
  /// ```
  /// let ipc_client = DiscordIpcClient::new("<some client id>")?;
  /// ```
  pub async fn new(client_id: &str, access_token: &str) -> Result<Self> {
    let socket = DiscordIpcSocket::new().await?;

    let mut client = Self {
      client_id: client_id.to_string(),
      access_token: access_token.to_owned(),
      userdata: None,
      socket,
      ready_callback: None
    };

    // connect to client
    client.connect().await.ok();
   
    // test auth client
    // client.authorize().await.ok();

    // login to cleint
    client.login().await?;

    Ok(client)
  }

  /// Connects the client to the Discord IPC.
  ///
  /// This method attempts to first establish a connection,
  /// and then sends a handshake.
  ///
  /// # Errors
  ///
  /// Returns an `Err` variant if the client
  /// fails to connect to the socket, or if it fails to
  /// send a handshake.
  ///
  /// # Examples
  /// ```
  /// let mut client = discord_ipc::new_client("<some client id>")?;
  /// client.connect()?;
  /// ```
  async fn connect(&mut self) -> Result<()> {
    println!("Connecting to client...");
    self.send_handshake().await?;

    let (_opcode, payload) = self.socket.recv().await.unwrap();

    println!("Connect raw {}", payload);
    // spooky line is not working
    let payload = serde_json::from_str(&payload)?;
    match payload {
      BasedEvent::Ready { data } => {
        println!("Connected to discord and got ready event! {:#?}", data);
        
        self.userdata = Some(data.user);
      }
      _ => {
        println!("Could not connect to discord...");
      }
    }

    Ok(())
  }

  /// Handshakes the Discord IPC.
  ///
  /// This method sends the handshake signal to the IPC.
  /// It is usually not called manually, as it is automatically
  /// called by [`connect`] and/or [`reconnect`].
  ///
  /// [`connect`]: #method.connect
  /// [`reconnect`]: #method.reconnect
  ///
  /// # Errors
  ///
  /// Returns an `Err` variant if sending the handshake failed.
  async fn send_handshake(&mut self) -> Result<()> {
    self
      .socket
      .send(
        &json!({
          "v": 1,
          "client_id": self.client_id
        })
        .to_string(),
        OPCODES::Handshake as u8,
      )
      .await?;

    Ok(())
  }

  /// Send authorize which does an in app popup modal
  pub async fn authorize(&mut self) -> Result<()> {
    let nonce = Uuid::new_v4().to_string();

    self
      .socket
      .send(
        &json!({
          "cmd": RPCCommand::Authorize,
          "args": {
            "client_id": self.client_id,
            "scopes": ["rpc", "identify"],
            "rpc_token": "ligma"
          },
          "nonce": nonce
        })
        .to_string(),
        OPCODES::Frame as u8,
      )
      .await?;

    self.socket.recv().await?;

    Ok(())
  }

  /// Send auth
  ///
  /// This method sends the auth token to the IPC.
  ///
  /// Returns an `Err` variant if sending the handshake failed.
  pub async fn login(&mut self) -> Result<()> {
    let nonce = Uuid::new_v4().to_string();

    // use token stored in instance
    let token = &self.access_token;

    println!("Logging in with token, {}", token);
    self
      .socket
      .send(
        &json!({
          "cmd": "AUTHENTICATE",
          "args": {
            "access_token": token
          },
          "nonce": nonce
        })
        .to_string(),
        OPCODES::Frame as u8,
      )
      .await?;

    self.socket.recv().await?;

    Ok(())
  }

  /// send a json string payload to the socket
  pub async fn emit_string(&mut self, payload: String) -> Result<()> {
    self.socket.send(&payload, OPCODES::Frame as u8).await.unwrap();
    Ok(())
  }

  // /// send a json string payload to the socket
  // pub async fn emit_command(&mut self, command: &RPCCommand) -> Result<()> {
  //   let mut command_json = command.to_json()?;
  //   let json_string = &create_json(&mut command_json)?;
  //   println!("Sending, {}", json_string);
  //   self
  //     .socket
  //     .send(json_string, OPCODES::Frame as u8)
  //     .await
  //     .unwrap();
  //   Ok(())
  // }

  pub async fn handle_ready<F>(&mut self, func: F)
  where
    F: Fn(EventReceive) + Send + Sync + 'static,
  {

  }

  pub async fn handler<F>(&mut self, func: F)
  where
    F: Fn(EventReceive) + Send + Sync + 'static,
  {
    let mut socket_clone = self.socket.clone();
    tokio::spawn(async move {
      loop {
        let (_opcode, payload) = socket_clone.recv().await.unwrap();
        println!("raw {}", payload);
        match serde_json::from_str::<EventReceive>(&payload) {
          Ok(e) => {
            // TODO: give the consumer a ready event so they can sub to events

            func(e);
          }
          Err(e) => {
            println!("{:#?}", e);
          }
        }
      }
    });
  }
}
