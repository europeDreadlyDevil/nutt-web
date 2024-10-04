use std::ops::DerefMut;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::net::TcpStream;

pub struct StreamReader<'a> {
    stream: &'a mut TcpStream,
}

impl<'a> StreamReader<'a> {
    pub fn new(stream: &'a mut TcpStream) -> StreamReader {
        Self { stream }
    }
    pub async fn read_req(&mut self) -> String {
        let mut buf_reader = BufReader::new(self.stream.deref_mut());
        let mut content_length = 0;
        let mut req = String::new();
        while let Ok(bytes) = buf_reader.read_line(&mut req).await {
            if bytes == 0 {
                break;
            }
            if req.ends_with("\r\n\r\n") {
                break;
            }
            if let Some(line) = req.find("Content-Length: ") {
                let len_str = &req[line + 15..];
                if let Some(end) = len_str.find("\r\n") {
                    content_length = len_str[..end].trim().parse::<usize>().unwrap();
                }
            }
        }
        if content_length > 0 {
            let mut body = vec![0; content_length];
            buf_reader.read_exact(&mut body).await.unwrap();
            req.push_str(String::from_utf8_lossy(&body).as_ref());
        }

        req
    }
}
