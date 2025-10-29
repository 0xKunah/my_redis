use std::time::Instant;
use rand::{Rng, rng};
mod tcp;

fn generate_random_message() -> String {
    let mut rng = rng();

    let cmd = match rng.random_range(0..3) {
        0 => "GET",
        1 => "SET",
        _ => "DEL",
    };

    let key = format!("myKey{}", rng.random_range(0..=100));

    match cmd {
        "SET" => {
            let value = format!("value{}", rng.random_range(1..=2147483647));
            format!("{} {} {}\n", cmd, key, value)
        }
        _ => format!("{} {}\n", cmd, key),
    }
}

fn main() -> std::io::Result<()> {
	let mut conn = tcp::TcpClient::new()?;
	let start = Instant::now();

	for _ in 0..100000 {
		conn.send_message(generate_random_message().as_str()).unwrap();
	}

	println!("{}ms", start.elapsed().as_millis());

	Ok(())
}