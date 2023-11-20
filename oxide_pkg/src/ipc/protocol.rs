use std::io;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use super::utils::{DeserializePacket, SerializePacket};

pub struct Protocol {
    reader: io::BufReader<TcpStream>,
    stream: TcpStream,
}

impl Protocol {
    /// Wrap a TcpStream with Protocol
    pub fn with_stream(stream: TcpStream) -> io::Result<Self> {
        Ok(Self {
            reader: io::BufReader::new(stream.try_clone()?),
            stream,
        })
    }

    /// Establish a connection, wrap stream in BufReader/Writer
    pub fn connect(dest: SocketAddr) -> io::Result<Self> {
        let stream = TcpStream::connect(dest)?;
        Self::with_stream(stream)
    }

    /// Serialize a message to the server and write it to the TcpStream
    pub fn send_message(&mut self, message: &impl SerializePacket) -> io::Result<()> {
        message.serialize(&mut self.stream)?;
        self.stream.flush()
    }

    /// Read a message from the inner TcpStream
    ///
    /// NOTE: Will block until there's data to read (or deserialize fails with io::ErrorKind::Interrupted)
    ///       so only use when a message is expected to arrive
    pub fn read_message<T: DeserializePacket>(&mut self) -> io::Result<T::Output> {
        T::deserialize(&mut self.reader)
    }
}
