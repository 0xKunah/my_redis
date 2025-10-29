mod tcp;

fn main() -> std::io::Result<()> {
	let mut conn = tcp::TcpClient::new()?;

	println!("Received response: {}", conn.send_message("GET myFirstKey").expect("Could not send message"));
	println!("Received response: {}", conn.send_message("SET myFirstKey 2").expect("Could not send message"));
	println!("Received response: {}", conn.send_message("GET myFirstKey").expect("Could not send message"));
	println!("Received response: {}", conn.send_message("DEL myFirstKey").expect("Could not send message"));
	println!("Received response: {}", conn.send_message("GET myFirstKey").expect("Could not send message"));

	Ok(())
}