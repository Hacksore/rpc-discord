use serde_json::Value;
use uuid::Uuid;
use std::convert::TryInto;
use std::error::Error;

pub fn create_json(mut value: serde_json::Value) -> String {
  let uuid = Uuid::new_v4().to_string();

  let payload = value.as_object_mut().unwrap();
  payload.insert("nonce".to_string(), Value::String(uuid));

  // TODO: RISKY NEED TO FIX ERROR HANDLING
  serde_json::to_string(&payload).unwrap()
}

// Re-implement some packing methods in Rust
pub fn pack(opcode: u32, data_len: u32) -> Result<Vec<u8>, Box<dyn Error>> {
  let mut bytes = Vec::new();

  for byte_array in &[opcode.to_le_bytes(), data_len.to_le_bytes()] {
    bytes.extend_from_slice(byte_array);
  }

  Ok(bytes)
}

pub fn unpack(data: Vec<u8>) -> Result<(u32, u32), Box<dyn Error>> {
  let data = data.as_slice();
  let (opcode, header) = data.split_at(std::mem::size_of::<u32>());

  let opcode = u32::from_le_bytes(opcode.try_into()?);
  let header = u32::from_le_bytes(header.try_into()?);

  Ok((opcode, header))
}
