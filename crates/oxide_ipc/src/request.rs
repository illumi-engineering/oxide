use std::io::{self, Read, Write};
use byteorder::{NetworkEndian, WriteBytesExt, ReadBytesExt};
use super::utils::{DeserializePacket, extract_string, SerializePacket};

pub enum IpcRequest {
    SyncProject { root_dir: String },
    // Registries {}
}

impl From<&IpcRequest> for u8 {
    fn from(req: &IpcRequest) -> Self {
        match req {
            IpcRequest::SyncProject { .. } => 1,
        }
    }
}

impl SerializePacket for IpcRequest {
    /// Serialize Request to bytes (to send to server)
    fn serialize(&self, buf: &mut impl Write) -> io::Result<usize> {
        buf.write_u8(self.into())?; // Message Type byte
        let mut bytes_written: usize = 1;
        match self {
            IpcRequest::SyncProject { root_dir } => {
                // Write the variable length message string, preceded by it's length
                let root_dir = root_dir.as_bytes();
                buf.write_u16::<NetworkEndian>(root_dir.len() as u16)?;
                buf.write_all(&root_dir)?;
                bytes_written += 2 + root_dir.len()
            }
        }
        Ok(bytes_written)
    }
}

impl DeserializePacket for IpcRequest {
    type Output = IpcRequest;

    fn deserialize(mut buf: &mut impl Read) -> io::Result<IpcRequest> {
        // We'll match the same `u8` that is used to recognize which request type this is
        match buf.read_u8()? {
            // Echo
            1 => Ok(IpcRequest::SyncProject { root_dir: extract_string(&mut buf)? }),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid Request Type",
            )),
        }
    }
}