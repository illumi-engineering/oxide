
use std::io::{self, Read, Write};
use byteorder::{NetworkEndian, WriteBytesExt, ReadBytesExt};
use super::utils::{DeserializePacket, extract_string, SerializePacket};

pub enum IpcResponse {
    SyncProject { ok: bool, changed: bool },
    InstallProject { ok: bool },
}

impl From<&IpcResponse> for u8 {
    fn from(req: &IpcResponse) -> Self {
        match req {
            IpcResponse::SyncProject { .. } => 1,
            IpcResponse::InstallProject { .. } => 2,
        }
    }
}

impl SerializePacket for IpcResponse {
    /// Serialize Response to bytes (to send to client)
    fn serialize(&self, buf: &mut impl Write) -> io::Result<usize> {
        buf.write_u8(self.into())?; // Message Type byte
        let mut bytes_written: usize = 1;
        match self {
            IpcResponse::SyncProject { ok, changed } => {
                buf.write_i8(*ok as i8)?;
                bytes_written += 1;

                buf.write_i8(*changed as i8)?;
                bytes_written += 1;

                // let message = message.as_bytes();
                // buf.write_u16::<NetworkEndian>(message.len() as u16)?;
                // buf.write_all(&message)?;
                // bytes_written += 2 + message.len();
            }
            IpcResponse::InstallProject { ok } => {
                buf.write_i8(*ok as i8)?;
                bytes_written += 1;
            }
        }
        Ok(bytes_written)
    }
}

impl DeserializePacket for IpcResponse {
    type Output = IpcResponse;

    fn deserialize(mut buf: &mut impl Read) -> io::Result<IpcResponse> {
        // We'll match the same `u8` that is used to recognize which response type this is
        match buf.read_u8()? {
            // Echo
            1 => Ok(IpcResponse::SyncProject {
                ok: buf.read_i8().unwrap() != 0,
                changed: buf.read_i8().unwrap() != 0,
                // message: extract_string(&mut buf)?,
            }),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid Request Type",
            )),
        }
    }
}