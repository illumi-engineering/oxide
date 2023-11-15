use std::io::{self, Read, Write};
use byteorder::{NetworkEndian, ReadBytesExt};

/// From a given readable buffer (TcpStream), read the next length (u16) and extract the string bytes ([u8])
pub fn extract_string(buf: &mut impl Read) -> io::Result<String> {
    let length = buf.read_u16::<NetworkEndian>()?;

    // Given the length of our string, only read in that quantity of bytes
    let mut bytes = vec![0u8; length as usize];
    buf.read_exact(&mut bytes)?;

    // And attempt to decode it as UTF8
    String::from_utf8(bytes).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid utf8"))
}

pub trait SerializePacket {
    /// Serialize to a `Write`able buffer
    fn serialize(&self, buf: &mut impl Write) -> io::Result<usize>;
}
/// Trait for something that can be converted from bytes (&[u8])
pub trait DeserializePacket {
    /// The type that this deserializes to
    type Output;

    /// Deserialize from a `Read`able buffer
    fn deserialize(buf: &mut impl Read) -> io::Result<Self::Output>;
}