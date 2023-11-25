use std::io::{self, Read, Write};
use byteorder::{NetworkEndian, WriteBytesExt, ReadBytesExt};
use super::utils::{DeserializePacket, extract_string, SerializePacket};

pub enum IpcRequest {
    SyncProject { root_dir: String },
    InstallProject { project_dir: String, workspace: bool }
}

impl From<&IpcRequest> for u8 {
    fn from(req: &IpcRequest) -> Self {
        match req {
            IpcRequest::SyncProject { .. } => 1,
            IpcRequest::InstallProject { .. } => 2,
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
            IpcRequest::InstallProject { project_dir, workspace } => {
                // Write the variable length message string, preceded by it's length
                let project_dir = project_dir.as_bytes();
                buf.write_u16::<NetworkEndian>(project_dir.len() as u16)?;
                buf.write_all(&project_dir)?;
                bytes_written += 2 + project_dir.len();

                buf.write_i8(*workspace as i8)?;
                bytes_written += 1;
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
            2 => Ok(IpcRequest::InstallProject {
                project_dir: extract_string(&mut buf)?,
                workspace: buf.read_i8().unwrap() != 0,
            }),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid Request Type",
            )),
        }
    }
}