use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::TcpStream;

pub struct Connection {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>
}

pub const READ_BUFFER_SIZE: usize = 256;
type ConditionFunction<'a> = &'a dyn Fn(&str) -> bool;

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
        self.write_buffer(string.as_bytes())?;
        Ok(())
    }

     pub fn write_string_slice(&mut self, string: &str) -> anyhow::Result<()> {
        self.write_buffer(string.as_bytes())?;
        Ok(())
    }

     pub fn flush_writer(&mut self) -> anyhow::Result<()> {
        self.writer.flush()?;
        Ok(())
    }

     pub fn read_buffer(&mut self, buffer: &mut [u8]) -> anyhow::Result<usize> {
        let amount = self.reader.read(buffer)?;
        Ok(amount)
    }

     pub fn read_string_until_condition(
        &mut self,
        condition_function: ConditionFunction,
    ) -> anyhow::Result<String> {
        let mut string_buffer = String::with_capacity(READ_BUFFER_SIZE * 3);

        let mut read_buffer = [0u8; READ_BUFFER_SIZE];
        loop {
            let read_amount = self.read_buffer(&mut read_buffer)?;
            let string = String::from_utf8((&read_buffer[..read_amount]).to_vec())?;
            string_buffer.push_str(&string);
            if condition_function(string_buffer.as_str()) {
                return Ok(string_buffer);
            }
        }
    }

     pub fn read_fully_into_string(&mut self) -> anyhow::Result<String> {
        let received: Vec<u8> = self.reader.fill_buf()?.to_vec();
        self.reader.consume(received.len());

        let string = String::from_utf8(received)?;
        Ok(string)
    }
}
