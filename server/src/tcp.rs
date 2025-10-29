use std::{collections::HashMap, io::{self, Read, Write}, net::{TcpListener, TcpStream}, sync::Arc, thread};

use crate::store::Store;

pub struct TcpServer {
	handlers: HashMap<String, Box<dyn (Fn(&Store, &[&str]) -> String) + Send + Sync>>,
	listener: TcpListener,
	store: Arc<Store>
}

impl TcpServer {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            listener: TcpListener::bind(("127.0.0.1", 34600))?,
			handlers: HashMap::new(),
			store: Store::new(),
        })
    }

	pub fn parse_message(&self, msg: &str) -> String {
		match msg.ends_with("\r\n") {
			true => return msg.to_string(),
			false => return format!("{} \r\n", msg),
		}
	}

	pub fn handle_client(&self, mut stream: TcpStream) {
		println!("Accepted connection {:?}", stream.peer_addr());
		let store = Arc::clone(&self.store);
		let mut buffer = [0; 512];

		loop {
			let n = match stream.read(&mut buffer) {
				Ok(0) | Err(_) => break,
				Ok(n) => n,
			};
			let input = String::from_utf8_lossy(&buffer[..n]);
			let parts: Vec<&str> = input.trim().split_whitespace().collect();
			if parts.is_empty() { continue; }

			let (command, args) = (parts[0], &parts[1..]);
			let mut response: String = String::new();

			if let Some(handler) = self.handlers.get(command) {
				response = handler(&store, args);
			} else {
				println!("Unknown command: {}", command);
				response.push_str("-ERR unknown command");
			}

			response = self.parse_message(&response);
			println!("Sending response: {}", response);
			stream.write_all(response.as_bytes()).unwrap();
		}

		println!("Closed connection {:?}", stream.peer_addr());
	}

	pub fn listen(self: Arc<Self>) {
		for stream in self.listener.incoming() {
			let server_ref = Arc::clone(&self);
			thread::spawn(move || server_ref.handle_client(stream.unwrap()));
		}
	}

	pub fn on_message<F>(&mut self, msg: &str, handler: F)
    where
        F: (Fn(&Store, &[&str]) -> String) + Send + Sync + 'static,
	{
		self.handlers.insert(msg.to_string(), Box::new(handler));
	}
}