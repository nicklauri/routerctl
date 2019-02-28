
use std::io::{Error, ErrorKind, Read, Write};
use std::net::{TcpStream, SocketAddr};

use super::commandline::ARGS;

pub fn get(request_string: &str) -> Result<String, Error> {
    Client::connect(ARGS.router)?.get(request_string)
}

pub fn post(request_string: &str, data: &str) -> Result<String, Error> {
    Client::connect(ARGS.router)?.post(request_string, data)
}

#[derive(Debug)]
pub struct Client {
    stream: TcpStream
}

impl Client {
    pub fn connect(addr: &str) -> Result<Self, Error> {
        Ok(Self {
            stream: match addr.parse::<SocketAddr>() {
                Ok(addr) => TcpStream::connect(addr)?,
                Err(e) => return Err(Error::new(ErrorKind::Other, e.to_string()))
            }
        })
    }

    pub fn get(&mut self, request: &str) -> Result<String, Error> {
        let request = Self::create_request_header(request, "");
        self.stream.write_all(request.as_bytes())?;
        self.read_stream()
    }

    pub fn post(&mut self, request: &str, data: &str) -> Result<String, Error> {
        let request = Self::create_request_header(request, data);
        self.stream.write_all(request.as_bytes())?;
        self.read_stream()
    }

    pub fn read_stream(&mut self) -> Result<String, Error> {
        // shutdown write stream make it faster! I dunno anymore
        self.stream.shutdown(std::net::Shutdown::Write)?;
        let mut buffer = String::new();
        self.stream.read_to_string(&mut buffer)?;
        Ok(buffer)
    }

    pub fn create_request_header(r: &str, d: &str) -> String {
        if !d.is_empty() {
            format!(concat!("POST {} HTTP/1.1\r\n",
                "Host: 192.168.1.1\r\n",
                "Accept: */*\r\n",
                "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) ",
                    "AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.113 Safari/537.36",
                "Connection: close\r\n\r\n{}",), r, d)
        }
        else {
            format!(concat!("GET {} HTTP/1.1\r\n",
                "Host: 192.168.1.1\r\n",
                "Accept: */*\r\n",
                "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) ",
                    "AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.113 Safari/537.36\r\n",
                "Connection: close\r\n\r\n",), r)
        }
    }
}
