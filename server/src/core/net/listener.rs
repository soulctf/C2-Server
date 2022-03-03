use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::thread;

#[path = "../config.rs"]
mod config;

#[path = "../log.rs"]
mod log;

#[path = "encryption.rs"]
mod encryption;

#[path = "session.rs"]
mod session;

// TODO: https://trello.com/c/FftuRBn1
pub fn init_host_listener(port: u16) -> Result<bool, io::Error> {
	let socket = create_socket(port);
	let listener = TcpListener::bind(socket).unwrap();
	// Loop to handle connections
	for stream in listener.incoming() {
		let stream = stream.unwrap();
		thread::spawn(|| {
			handle_client_connection(stream);
		});
	}
	return Ok(true);
}

// TODO: https://trello.com/c/Tk2AFl04
fn handle_client_connection(connection: TcpStream) {
	log::log("", connection);
}

// TODO: https://trello.com/c/p8NmLJxO
fn activate_connection(connection: TcpStream) {
	log::log("", connection);
}

// TODO: https://trello.com/c/Q9sxhqQH
fn is_allowed_ip(ip: Ipv4Addr) -> bool {
	return false;
}

// TODO: https://trello.com/c/XWKCgSqY
fn create_socket(port: u16) -> SocketAddr {
	return SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
}