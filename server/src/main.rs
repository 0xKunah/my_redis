use std::sync::Arc;

use crate::{tcp::TcpServer};
mod store;
mod tcp;

fn main() -> std::io::Result<()> {
	let mut server = TcpServer::new()?;

	server.on_message("PING", |_, args| {
		println!("Received ping message, {:?}", args);

		"+PONG".to_string()
	});

	server.on_message("GET", |store, args| {
		store.get(args[0])
	});

	server.on_message("SET", |store, args| {
		store.set(args[0], args[1]);

		// S means success, only the first letter is sent for performance
		"S".to_string()
	});

	server.on_message("DEL", |store, args| {
		store.del(args[0]);

		// S means success, only the first letter is sent for performance
		"S".to_string()
	});

	Arc::new(server).listen();
    Ok(())
}