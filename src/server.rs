use std::net::SocketAddr;

use message_io::node;
use message_io::network::{NetEvent, Transport};

#[derive(Debug, Clone)]
pub struct Client {
	pub address: SocketAddr,
	pub name: String,

}

impl Client {
	pub fn new(name: String, address: SocketAddr) -> Self {
		Client { address: address, name: name }
	}
}

pub fn main() {
	let (handler, listener) = node::split::<()>();

	handler.network().listen(Transport::FramedTcp, "0.0.0.0:9009").unwrap();

	listener.for_each(move |event| match event.network() {
		NetEvent::Connected(_, _) => unreachable!(),
		NetEvent::Accepted(_endpoint, _listener) => println!("Client connected"),
		NetEvent::Message(endpoint, data) => {
			println!("Received: {}", String::from_utf8_lossy(data));
			handler.network().send(endpoint, data);
		},
		NetEvent::Disconnected(_endpoint) => println!("Client disconnected"),
	});
}