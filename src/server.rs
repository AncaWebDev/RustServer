use std::net::TcpListener;
use std::convert::TryFrom;
use std::convert::TryInto;
use crate::http::Request;
use std::io::Read;

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

                            match Request::try_from(&buffer[..]){
                                Ok(request) => {} ,
                                Err(e) => println!("error: {}", e)
                            }

                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    };
                }
                Err(e) => println!("error: {}", e)
            }
        }

        
    }

}
