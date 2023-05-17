use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::TcpStream;

pub struct Connection {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>
}

impl Connection {
    pub fn connect(address: &str) -> anyhow::Result<Self> {
        let stream = TcpStream::connect(address)?;
        let reader = BufReader::new(stream.try_clone()?);
        let writer = BufWriter::new(stream);
        Ok(Self {
            reader, writer
        })
    }

    pub fn write_buffer(&mut self, buffer: &[u8]) -> anyhow::Result<()> {
        self.writer.write(buffer)?;
        Ok(())
    }

    pub fn write_string(&mut self, string: String) -> anyhow::Result<()> {
        self.writer.write(string.as_bytes())?;
        Ok(())
    }

    pub fn write_string_slice(&mut self, string: &str) -> anyhow::Result<()> {
        self.writer.write(string.as_bytes())?;
        Ok(())
    }

    pub fn flush_writer(&mut self) -> anyhow::Result<()> {
        self.writer.flush()?;
        Ok(())
    }

    pub fn read_fully_into_string(&mut self) -> anyhow::Result<String> {
        let received: Vec<u8> = self.reader.fill_buf()?.to_vec();
        self.reader.consume(received.len());

        let string = String::from_utf8(received)?;
        Ok(string)
    }
}
