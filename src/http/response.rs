use super::StatusCode;
//use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Result as IoResult;
use tokio::io::AsyncWriteExt;
//use tokio::net::TcpStream;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    pub async fn send(&self, stream: &mut tokio::net::TcpStream) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        //let response = "HTTP/1.1 200 OK";

        stream
            .write_all(
                format!(
                    "HTTP/1.1 {} {}\r\n\r\n{}",
                    self.status_code,
                    self.status_code.reason_phrase(),
                    body
                )
                .as_bytes(),
            )
            .await
            .unwrap();
        //stream.write_all(response.as_bytes()).await.unwrap();
        Ok(())

        /*
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
        */
    }
}
