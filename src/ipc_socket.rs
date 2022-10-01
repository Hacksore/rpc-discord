use std::sync::Arc;

use crate::Result;

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

use crate::{pack, unpack};

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
  #[cfg(target_os = "windows")]
  async fn get_inner_socket() -> Result<(ReadHalfType, WriteHalfType)> {
    for i in 0..10 {
      let name = format!(r"\\?\pipe\discord-ipc-{}", i);
      match ClientOptions::new().open(&name) {
        Ok(client) => {
          let (read_half, write_half) = tokio::io::split(client);
          return Ok((read_half, write_half));
        }
        Err(_) => continue,
      }
    }

    return Err(eyre!("Couldn't connect to the Discord IPC socket"));
  }

  #[cfg(target_family = "unix")]
  async fn get_inner_socket() -> Result<(ReadHalfType, WriteHalfType)> {
    use crate::{errors::DiscordRPCError, get_pipe_pattern};

    for i in 0..10 {
      let path = get_pipe_pattern().join(format!("discord-ipc-{}", i));

      match UnixStream::connect(&path).await {
        Ok(socket) => {
          return Ok(socket.into_split());
        }
        Err(_) => continue,
      }
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
