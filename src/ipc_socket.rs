use crate::Result;
use std::sync::Arc;

#[cfg(target_family = "unix")]
use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};

#[cfg(target_family = "unix")]
use tokio::net::UnixStream;
use tokio::{
  io::{AsyncReadExt, AsyncWriteExt},
  sync::Mutex,
};

#[cfg(target_family = "windows")]
use tokio::{
  io::{ReadHalf, WriteHalf},
  net::windows::named_pipe::{ClientOptions, NamedPipeClient},
};

use crate::{errors::DiscordRPCError, get_pipe_pattern, pack, unpack};

#[cfg(target_family = "windows")]
type ReadHalfType = ReadHalf<NamedPipeClient>;
#[cfg(target_family = "windows")]
type WriteHalfType = WriteHalf<NamedPipeClient>;

#[cfg(target_family = "unix")]
type ReadHalfType = OwnedReadHalf;
#[cfg(target_family = "unix")]
type WriteHalfType = OwnedWriteHalf;

#[derive(Clone)]
pub(crate) struct DiscordIpcSocket {
  read_half: Arc<Mutex<ReadHalfType>>,
  write_half: Arc<Mutex<WriteHalfType>>,
}

impl DiscordIpcSocket {
  /// Used to get the a socket like impl on windows as technical it's a named pipe
  #[cfg(target_os = "windows")]
  async fn get_inner_socket() -> Result<(ReadHalfType, WriteHalfType)> {
    let path = get_pipe_pattern();
    if let Ok(client) = ClientOptions::new().open(&path) {
      let (read_half, write_half) = tokio::io::split(client);
      return Ok((read_half, write_half));
    }

    Err(DiscordRPCError::CouldNotConnect)
  }

  #[cfg(target_family = "unix")]
  async fn get_inner_socket() -> Result<(ReadHalfType, WriteHalfType)> {
    let path = get_pipe_pattern();

    if let Ok(socket) = UnixStream::connect(&path).await {
      return Ok(socket.into_split());
    }

    Err(DiscordRPCError::CouldNotConnect)
  }

  pub(crate) async fn new() -> Result<Self> {
    let (read_half, write_half) = Self::get_inner_socket().await?;
    Ok(Self {
      read_half: Arc::new(Mutex::new(read_half)),
      write_half: Arc::new(Mutex::new(write_half)),
    })
  }

  pub(crate) async fn write(&mut self, buf: &[u8]) -> Result<()> {
    let mut socket = self.write_half.lock().await;
    socket.write_all(buf).await?;
    Ok(())
  }

  pub(crate) async fn read(&mut self, buf: &mut [u8]) -> Result<()> {
    let mut socket = self.read_half.lock().await;
    socket.read_exact(buf).await?;
    Ok(())
  }

  pub(crate) async fn send(&mut self, data: &str, opcode: u8) -> Result<()> {
    let mut packet = pack(opcode.into(), data.len() as u32)?;

    packet.extend(data.as_bytes());

    self.write(&packet).await?;

    Ok(())
  }

  pub(crate) async fn recv(&mut self) -> Result<(u32, String)> {
    let mut header = [0u8; 8];
    self.read(&mut header).await?;
    let (op, length) = unpack(header.to_vec())?;

    let mut data = vec![0u8; length as usize];
    self.read(&mut data).await?;

    let response = String::from_utf8(data)?;

    Ok((op, response))
  }
}
