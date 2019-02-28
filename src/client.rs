// MIT License

// Copyright (c) 2019 Nick Lauri

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
