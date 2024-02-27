use message_io::node::{self, NodeEvent};
use message_io::network::{NetEvent, Transport};

use std::io::Write;
use std::sync::{Arc, Mutex, Barrier};

use crate::ident::Identification;
use crate::util::get_input;

enum Signal {
	Greet,
	Message(String)
}

pub fn main(host: String, name: String) {
    let prompt: String = format!("rustchat@{} > ",host);

	let (handler, listener) = node::split();

    let handler = Arc::new(Mutex::new(handler));

	let (server, _) = match handler.lock().unwrap().network().connect(Transport::FramedTcp, host.clone() + ":9009") {
		Ok(c) => {
			println!("Connecting to {}", host);
			c
		},
		Err(e) => {
			eprintln!("{}", e);
			return;
		}
	};

    let mut ident: Identification = Identification::new(0, name, true);

    let handler_ = Arc::clone(&handler);

    let startup_barrier: Arc<Barrier> = Arc::new(Barrier::new(2));

    let c_barrier: Arc<Barrier> = Arc::clone(&startup_barrier);
    let prompt_c: String = prompt.clone();

	let event_thread = std::thread::spawn(move || listener.for_each(move |event| {let handler = &handler_; match event {
        NodeEvent::Network(net_event) => match net_event {
            NetEvent::Connected(endpoint, ok) => {
                if ok {
                    println!("Connection to {} established!", endpoint.addr());
                    handler.lock().unwrap().signals().send(Signal::Greet);
                } else {
                    println!("Connection failed.");
                    std::process::exit(1);
                }
            },
            NetEvent::Accepted(_, _) => unreachable!(), // Only generated by listening
            NetEvent::Message(_endpoint, data) => {
                let data = String::from_utf8_lossy(data).to_string();
                if ident.helo {
                    ident.helo = false;
                    ident.id = match u64::from_str_radix(&data, 10) {
                        Ok(id) => id,
                        Err(e) => {
                            eprintln!("Error occurred: {:#?}", e);
                            handler.lock().unwrap().stop();
                            return
                        },
                    };

                    c_barrier.wait();
                    return
                }

                let data: Vec<&str> = data.split(";").collect();

                if data[0] == "MSG" {
                    if let Some(msg) = data.get(1) {
                        print_message(prompt_c.as_str(), msg.to_string());
                    }
                } else if data[0] == "NEW" {
                    if let Some(name) = data.get(1) {
                        print_message(prompt_c.as_str(), format!("{} joined the chat!",name));
                    }
                } else if data[0] == "DIS" {
                    if let Some(name) = data.get(1) {
                        print_message(prompt_c.as_str(), format!("{} left the chat.",name));
                    }
                }
            },
            NetEvent::Disconnected(_endpoint) => (),
        }
        NodeEvent::Signal(signal) => match signal {
            Signal::Greet => {
                handler.lock().unwrap().network().send(server, &ident.to_ident_string());
            },
            Signal::Message(a) => {
                let mut data = ident.to_ident_string();
                data.append(&mut (";".to_owned() + &a).into_bytes());
                handler.lock().unwrap().network().send(server, &data.as_slice());
            }
        }
    }}));

    startup_barrier.wait();
    loop {
        let input = get_input(&prompt);

        if input.is_empty() {
            handler.lock().unwrap().stop();
            break;
        } else if input == "\n" {
            continue;
        }

        handler.lock().unwrap().signals().send(Signal::Message(input));
    }

    let _ = event_thread.join();
    
}

fn print_message(prompt: &str, message: String) {
    print!("\r{}"," ".repeat(prompt.len()+message.len()));
    let _ = std::io::stdout().flush();
    print!("\r{}\n",message);
    let _ = std::io::stdout().flush();
    print!("{}", prompt);
    let _ = std::io::stdout().flush();
}