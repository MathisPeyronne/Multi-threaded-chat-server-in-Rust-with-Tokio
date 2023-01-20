use crate::http::{ParseError, Request, Response, StatusCode};
use crate::server;
use async_trait::async_trait;
use futures::lock::Mutex;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::time::{sleep, Duration};
use tokio::{
    io::{AsyncRead, AsyncReadExt},
    net::TcpListener,
};

//for the database
//use crate::lib::run_database;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;

#[async_trait]
pub trait Handler {
    async fn handle_request(
        &mut self,
        request: &Request,
        conversationDatabase: Arc<Mutex<Vec<(String, String)>>>,
        ip_addr: String,
    ) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse a request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
    conversationDatabase: Arc<Mutex<Vec<(String, String)>>>,
}

impl Server {
    pub fn new(addr: String, conversationDatabase: Arc<Mutex<Vec<(String, String)>>>) -> Self {
        Self {
            addr,
            conversationDatabase,
        }
    }
    #[tokio::main]
    pub async fn run(self, mut handler: (impl Handler + Send + Clone + 'static)) {
        println!("launching database");
        dotenv().ok(); //loads the .env file
        let database_uri = dotenv!("DATABASE_URL");
        server_practice::run_database(database_uri).await;

        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).await.unwrap();

        let neww = Arc::new(Mutex::new(5));

        loop {
            let neww = neww.clone();
            let server_addr = self.conversationDatabase.clone();
            let mut handler = handler.clone();
            match listener.accept().await {
                Ok((mut stream, _)) => {
                    tokio::spawn(async move {
                        let mut buffer = [0; 1024];
                        //just testing
                        // stream
                        //     .write_all("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n".as_bytes())
                        //     .await
                        //     .unwrap();
                        // println!("here");
                        // sleep(Duration::from_millis(100000000000)).await;
                        // println!("end of sleep");
                        //
                        match stream.read(&mut buffer).await {
                            Ok(_) => {
                                println!(
                                    "Received a request: {}",
                                    String::from_utf8_lossy(&buffer)
                                );
                                let m = neww.lock().await;
                                println!("{:?}", m);
                                std::mem::drop(m);
                                let response = match Request::try_from(&buffer[..]) {
                                    Ok(request) => {
                                        handler
                                            .handle_request(
                                                &request,
                                                server_addr,
                                                stream.peer_addr().unwrap().to_string(),
                                            )
                                            .await
                                    }
                                    Err(e) => handler.handle_bad_request(&e),
                                };

                                if let Err(e) = response.send(&mut stream).await {
                                    println!("Failed to read from connection: {}", e);
                                }
                            }
                            Err(e) => println!("Failed to establish a connection: {}", e),
                        }
                    });
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
