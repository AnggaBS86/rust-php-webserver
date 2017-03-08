use std::net::{Shutdown, TcpListener};
use std::thread;
use std::io::Write;
use std::process::Command;

/**
* Very simple php webserver written in Rust
* Author : Angga Bayu S
* anggabs86@gmail.com
*/ 
fn main() {
 	   let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	   println!("Server initialized on port 8080");
 	   for stream in listener.incoming() {
        	thread::spawn(move || {
	   		let output = Command::new("php").arg("-r").output().expect("failed to execute process");
			let result = String::from_utf8_lossy(&output.stdout);
            		let mut stream = stream.unwrap();
            		match stream.write(result.as_bytes()) {
                		Ok(_) => println!("Response sent!"),
                		Err(e) => println!("Failed sending response: {}!", e),
            		}
            		stream.shutdown(Shutdown::Write).unwrap();
        	});
    }
}
