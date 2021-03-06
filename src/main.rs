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
	   		let output = Command::new("php").arg("-q").arg("php-doc-root/test.php").output().expect("failed to execute process");
			let result = String::from_utf8_lossy(&output.stdout.as_slice()).to_string();
			let mut header = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n".to_string();
			header.push_str(&result);
            		let mut stream = stream.unwrap();
            		match stream.write(header.as_bytes()) {
                		Ok(_) => println!("Response succesfully send by server"),
                		Err(error) => println!("Error occured : {}!", error),
            		}
            		stream.shutdown(Shutdown::Write).unwrap();
        	});
    }
}
