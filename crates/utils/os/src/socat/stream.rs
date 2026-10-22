use tokio::net::UnixStream;
use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};

pub struct SocatStream {
    pub reader: OwnedReadHalf,
    pub writer: OwnedWriteHalf,
    pub socket_path: String,
}

impl SocatStream {
    pub fn new(stream: UnixStream, socket_path: impl Into<String>) -> Self {
        let (reader, writer) = stream.into_split();
        Self {
            reader,
            writer,
            socket_path: socket_path.into(),
        }
    }

    pub fn into_split(self) -> (OwnedReadHalf, OwnedWriteHalf) {
        (self.reader, self.writer)
    }
}
