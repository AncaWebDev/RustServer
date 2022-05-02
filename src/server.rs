use std::net::TcpListener;
use std::convert::TryFrom;
use std::convert::TryInto;
use crate::http::{Request, Response, StatusCode};
use std::io::{Read, Write};

pub struct Server {
    addr: String,
}

impl Server{
    pub fn new(addr: String) -> Self {
        Server {
            addr
        }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("connection open");

        loop{
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer){
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]){
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(StatusCode::Ok, Some("<h1> IT WORKS </h1>".to_string()))
                                } 
                                Err(e) => {
                                    println!("error: {}", e);
                                    Response::new(StatusCode::BadRquest, None)
                                }
                            };

                            if let Err(e) = response.send(&mut stream){
                                println!("Failed to send response: {}", e);
                            }

                        }
                        Err(e) => {println!("Failed to read from connection: {}", e)},
                    };
                }
                Err(e) => println!("error: {}", e)
            }
        }

        
    }

}
