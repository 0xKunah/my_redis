use std::{thread, time::Instant};
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
	let mut handles = vec![];

	for i in 0..20 {
		let handle = thread::spawn(move || {
			let thread_tart = Instant::now();
			let mut conn = tcp::TcpClient::new().unwrap();

			for _ in 0..1000000 {
				conn.send_message(generate_random_message().as_str()).unwrap();
			}

			println!("Thread {} {}ms", i, thread_tart.elapsed().as_millis());
		});

		handles.push(handle);
	}

	for handle in handles {
        handle.join().unwrap();
    }

	Ok(())
}