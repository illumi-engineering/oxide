
use std::io::{self, Read, Write};
use byteorder::{NetworkEndian, WriteBytesExt, ReadBytesExt};
use crate::ipc::utils::{DeserializePacket, extract_string, SerializePacket};

pub enum LocalResponse {
    SyncProject { ok: bool, changed: bool },
}

impl From<&LocalResponse> for u8 {
    fn from(req: &LocalResponse) -> Self {
        match req {
            LocalResponse::SyncProject { .. } => 1,
        }
    }
}

impl SerializePacket for LocalResponse {
    /// Serialize Response to bytes (to send to client)
    fn serialize(&self, buf: &mut impl Write) -> io::Result<usize> {
        buf.write_u8(self.into())?; // Message Type byte
        let mut bytes_written: usize = 1;
        match self {
            LocalResponse::SyncProject { ok, changed } => {
                buf.write_i8(*ok as i8)?;
                bytes_written += 1;

                buf.write_i8(*changed as i8)?;
                bytes_written += 1;

                // let message = message.as_bytes();
                // buf.write_u16::<NetworkEndian>(message.len() as u16)?;
                // buf.write_all(&message)?;
                // bytes_written += 2 + message.len();
            }
        }
        Ok(bytes_written)
    }
}

impl DeserializePacket for LocalResponse {
    type Output = LocalResponse;

    fn deserialize(mut buf: &mut impl Read) -> io::Result<LocalResponse> {
        // We'll match the same `u8` that is used to recognize which response type this is
        match buf.read_u8()? {
            // Echo
            1 => Ok(LocalResponse::SyncProject {
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