use std::{collections::HashMap, fs::{File, OpenOptions}, io::{Read, Write}, sync::{Arc, RwLock, mpsc::{self, Sender}}, thread};

pub struct StoreData {
    pub data: HashMap<String, String>,
}

pub struct Store {
    inner: RwLock<StoreData>,
	tx: Sender<String>
}

impl Store {
    pub fn new() -> Arc<Self> {
		let (tx, rx) = mpsc::channel::<String>();

        let store = Arc::new(Self {
			inner: RwLock::new(StoreData { data: HashMap::new() }),
			tx: tx,
		});

		store.replay_aof();

		thread::spawn(move || {
			let mut f = OpenOptions::new().create(true).append(true).open("store.aof").unwrap();

			for line in rx {
				f.write(line.as_bytes()).unwrap();
			}
		});

		store
    }

	fn replay_aof(&self){
		let f: Option<File> = match OpenOptions::new().read(true).open("store.aof") {
			Ok(file) => Some(file),
			Err(_) => None,
		};

		if f.is_none() {
			return;
		}

		let mut inner = self.inner.write().expect("Unable to write");
		let mut content = String::new();

		if !f.expect("File doesnt exist").read_to_string(&mut content).is_ok() {
			return;
		}

		for line in content.lines() {
			let parts: Vec<&str> = line.split_whitespace().collect();

			match parts[0] {
				"SET" => { inner.data.insert(parts[1].to_string(), parts[2].to_string()); },
				"DEL" => { inner.data.remove(parts[1]); },
				_ => {}
			};
		}
	}

	pub fn get(&self, key: &str) -> String {
		match self.inner.read().expect("Unable to read").data.get(key).cloned() {
			Some(result) => result,
			None => "".to_string()
		}
	}

	pub fn set(&self, key: &str, value: &str) {
		self.inner.write().expect("Unable to write").data.insert(key.to_string(), value.to_string());
		self.tx.send(format!("SET {} {}\n", key, value)).unwrap();
	}

	pub fn del(&self, key: &str) {
		self.inner.write().expect("Unable to write").data.remove(key);
		self.tx.send(format!("DEL {}\n", key)).unwrap();
	}
}