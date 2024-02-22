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
		NetEvent::Accepted(endpoint, listener) => {
			println!("New connection from {}", endpoint.addr());
			clients.push(Client::new("UNREGISTERED".to_string(), endpoint.addr(), endpoint));
		},
		NetEvent::Message(endpoint, data) => {
			let msg = String::from_utf8_lossy(data);
			println!("Received: {}", msg);
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
			let ident: Identification = match Identification::from(msg.to_string()) {
				Some(i) => i,
				None => {
					handler.network().send(endpoint, b"Invalid IDENT");
					handler.network().remove(endpoint.resource_id());
					return
				}
			};
			if !client.heloed {
				/* if msg.contains("HELO FROM") {
					let name = match msg.find("|") {
						Some(i) => {
							Some(msg[..i].to_string())
						},
						None => {
							handler.network().send(endpoint, b"Invalid HELO");
							handler.network().remove(endpoint.resource_id());
							None
						}
					};
					if name.is_some() {
						client.name = name.unwrap();
						client.heloed = true;
						println!("{:?}", client);
					} else {
						handler.network().send(endpoint, b"Error: No name specified.");
						handler.network().remove(endpoint.resource_id());
					}
				} */
				if ident.helo {
					client.name = ident.name;
					client.heloed = true;
					handler.network().send(endpoint, format!("{}", client.id).as_bytes());
					println!("{:?}", client);
				} else {
					handler.network().send(endpoint, b"HELO first");
					handler.network().remove(endpoint.resource_id());
				}
			}
			// handler.network().send(endpoint, data);
		},
		NetEvent::Disconnected(_endpoint) => println!("Client disconnected"),
	});

	println!("hi");
}