use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};

use message_io::node;
use message_io::network::{Endpoint, NetEvent, Transport};

use crate::ident::Identification;

static USER_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone)]
pub struct Client {
	pub id: u64,
	pub address: SocketAddr,
	pub name: String,
	pub endpoint: Endpoint,
	pub heloed: bool // did the client introduce himself?
}

impl Client {
	pub fn new(name: String, address: SocketAddr, endpoint: Endpoint) -> Self {
		Client { id: USER_ID_COUNTER.fetch_add(1, Ordering::SeqCst), address: address, name: name, endpoint: endpoint, heloed: false }
	}
}

pub fn main() {
	let (handler, listener) = node::split::<()>();
	let mut clients: Vec<Client> = Vec::new();

	handler.network().listen(Transport::FramedTcp, "0.0.0.0:9009").unwrap();

	listener.for_each(move |event| match event.network() {
		NetEvent::Connected(_, _) => unreachable!(),
		NetEvent::Accepted(endpoint, _listener) => {
			println!("New connection from {}", endpoint.addr());
			clients.push(Client::new("UNREGISTERED".to_string(), endpoint.addr(), endpoint));
		},
		NetEvent::Message(endpoint, data) => {
			let received = String::from_utf8_lossy(data);
			println!("Received: {}", received);
			let client_index = clients.iter().position(|c| c.address == endpoint.addr()).unwrap();
			let client = match clients.get_mut(client_index) {
				Some(c) => c,
				None => {
					eprintln!("An error occurred.");
					handler.network().send(endpoint, b"An error occurred.");
					handler.network().remove(endpoint.resource_id());
					return;
				}
			};
			let ident: Identification = match Identification::from(received.to_string()) {
				Some(i) => i,
				None => {
					handler.network().send(endpoint, b"Invalid IDENT");
					handler.network().remove(endpoint.resource_id());
					return
				}
			};
			if !client.heloed {
				if ident.helo {
					client.name = ident.name;
					client.heloed = true;
					handler.network().send(endpoint, format!("{}", client.id).as_bytes());
					println!("{:?}", client);
					let new_name = client.name.clone();
					for client in &mut clients {
						handler.network().send(client.endpoint, format!("NEW;{}",new_name).as_bytes());
					}
					return
				} else {
					handler.network().send(endpoint, b"HELO first");
					handler.network().remove(endpoint.resource_id());
				}
			}

			if client.id != ident.id {
				handler.network().send(endpoint, b"Invalid authentication!");
				handler.network().remove(endpoint.resource_id());
				return
			}

			let message: String = {
				let a = received.to_string();
				let b: Vec<&str> = a.split(";").collect();
				let c = b.get(2);
				if c.is_some() {
					c.unwrap().to_string()
				} else {
					String::new()
				}
			};

			let sender_name = client.name.clone();

			for client in &mut clients {
				handler.network().send(client.endpoint, format!("MSG;{}: {}",sender_name,message).as_bytes());
			}
		},
		NetEvent::Disconnected(endpoint) => {
			println!("Client disconnected");
			let client_index = clients.iter().position(|c| c.address == endpoint.addr()).unwrap();
			let client = match clients.get_mut(client_index) {
				Some(c) => c,
				None => {
					eprintln!("An error occurred.");
					handler.network().send(endpoint, b"An error occurred.");
					handler.network().remove(endpoint.resource_id());
					return;
				}
			};
			let disconnected_name = client.name.clone();
			for client in &mut clients {
				handler.network().send(client.endpoint, format!("DIS;{}",disconnected_name).as_bytes());
			}
		},
	});

	println!("hi");
}