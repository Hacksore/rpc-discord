use std::{array::TryFromSliceError, string::FromUtf8Error};

use thiserror::Error;
use tokio::io;

#[derive(Error, Debug)]
pub enum DiscordRPCError {
  #[error("Could not find the IPC pipe")]
  PipeNotFound,
  #[error("Could not connect to Discord")]
  CouldNotConnect,
  #[error("Failed to convert from slice")]
  TryFromSlice(#[from] TryFromSliceError),
  #[error("An I/O error occurred")]
  Io(#[from] io::Error),
  #[error("Failed to convert UTF-8 bytes to String")]
  FromUtf8(#[from] FromUtf8Error),
  #[error("A serde_json error occurred")]
  SerdeJson(#[from] serde_json::Error),
}
