use std::{io::{self, BufRead, BufReader, Write}, net::TcpStream};

pub struct TcpClient {
	connection: TcpStream,
	reader: BufReader<TcpStream>,
}

impl TcpClient {
	pub fn new() -> io::Result<Self> {
        let stream = TcpStream::connect(("127.0.0.1", 34600))?;
        let reader = BufReader::new(stream.try_clone()?);

        Ok(Self {
            connection: stream,
            reader,
        })
    }

	pub fn parse_message(&self, msg: &str) -> Result<String, io::Error> {
		let parts: Vec<&str> = msg.trim().split_whitespace().collect();

		match parts.as_slice() {
			["PING"] => Ok("PING\r\n".to_string()),
			["GET", key] => Ok(format!("GET {}\r\n", key)),
			["SET", key, value] => Ok(format!("SET {} {}\r\n", key, value)),
			["DEL", key] => Ok(format!("DEL {}\r\n", key)),
			_ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown command")),
		}
	}

	pub fn read_response(&mut self) -> io::Result<String> {
		let mut response = String::new();

		self.reader.read_line(&mut response).expect("Could not read");

		Ok(response)
	}

	pub fn send_message(&mut self, msg: &str) -> io::Result<String> {
		let message = self.parse_message(msg).expect("Invalid message");

		self.connection.write_all(message.as_bytes())?;
		self.connection.flush()?;

		Ok(self.read_response().expect("Could not read"))
	}
}