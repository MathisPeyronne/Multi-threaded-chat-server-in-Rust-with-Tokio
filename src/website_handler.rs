use crate::http::QueryStringValue;
use async_trait::async_trait;

use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use futures::lock::Mutex;
use std::fs;
use std::sync::Arc;

#[derive(Clone)]
pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        //let path = format!("{}\\{}", self.public_path, file_path);

        //println!("{}", &self.public_path);
        //println!("{}", &fs::canonicalize(format!("public\\{}",  file_path)).unwrap().to_string_lossy()[4..]);
        //println!("{:?}", fs::canonicalize(format!("public/{}",  file_path)).unwrap().to_string_lossy()[4..].starts_with(&self.public_path));
        match fs::canonicalize(format!("public/{}", file_path)) {
            Ok(path) => {
                if path.to_string_lossy()[4..].starts_with(&self.public_path) {
                    println!("success: {}", format!("public/{}", file_path));
                    fs::read_to_string(path).ok()
                } else {
                    println!("{:?}", &path.to_string_lossy()[4..]);
                    println!("{:?}", self.public_path);
                    println!("Directory traversal attack attempt: {:?}", &path);
                    None
                }
            }
            Err(e) => {
                println!("error: {}", format!("public/{}", file_path));
                None
            }
        }
    }
}

#[async_trait]
impl Handler for WebsiteHandler {
    async fn handle_request(
        &mut self,
        request: &Request,
        conversationDatabase: Arc<Mutex<Vec<(String, String)>>>,
        ipAddr: String,
    ) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => {
                    //println!("{:?}", request.query_string().unwrap().data["message"]);
                    let mut convDatabase = conversationDatabase.lock().await;

                    match &request.query_string() {
                        Some(queryString) => {
                            //let queryStringData = &request.query_string().unwrap().data;
                            let username = match &request.query_string().unwrap().data["username"] {
                                QueryStringValue::Single(x) => {
                                    let x =
                                        x.replace("+", " ").replace("%27", "'").replace("%2C", ",");
                                    x
                                }
                                QueryStringValue::Multiple(vect) => vect[0].replace("+", " "),
                            };
                            match &request.query_string().unwrap().data["message"] {
                                QueryStringValue::Single(x) => {
                                    println!("{}", x);
                                    let x = x
                                        .replace("+", " ")
                                        .replace("%27", "'")
                                        .replace("%2C", ",")
                                        .replace("%3F", "?")
                                        .replace("%3A", ":")
                                        .replace("%2F", "/");
                                    convDatabase.push((x.to_string(), username.to_string()));
                                }
                                QueryStringValue::Multiple(vect) => {
                                    println!("im in heree");
                                    for vec in vect.iter() {
                                        convDatabase.push((vec.to_string(), username.to_string()));
                                    }
                                }
                            }
                            println!("{:?}", &request.query_string().unwrap().data["username"]);
                            for conv in convDatabase.iter() {
                                println!("{:?}", conv.0);
                                println!("{:?}", conv.1);
                            }
                            let mut chat_history_str = "".to_string();
                            let str_template = "<div> {ip}: {message} </div> \n";
                            for conv in convDatabase.iter() {
                                let str_buf = str_template.replace("{ip}", &conv.1);
                                chat_history_str += &str_buf.replace("{message}", &conv.0);
                                //.push_str + ": " + conv.0 + "</div> \n"
                                //chat_history_str = "fjkdlsjflk".to_string() + "jfkldsjlk" ;
                            }
                            let template =
                                self.read_file("hello.html").expect("failed to load file");
                            let html_str = template.replace("{CHAT_HISTORY}", &chat_history_str);
                            Response::new(StatusCode::Ok, Some(html_str))
                        }
                        None => {
                            let mut chat_history_str = "".to_string();
                            let str_template = "<div> {ip}: {message} </div> \n";
                            for conv in convDatabase.iter() {
                                let str_buf = str_template.replace("{ip}", &conv.1);
                                chat_history_str += &str_buf.replace("{message}", &conv.0);
                                //.push_str + ": " + conv.0 + "</div> \n"
                                //chat_history_str = "fjkdlsjflk".to_string() + "jfkldsjlk" ;
                            }
                            let template =
                                self.read_file("hello.html").expect("failed to load file");
                            let html_str = template.replace("{CHAT_HISTORY}", &chat_history_str);
                            Response::new(StatusCode::Ok, Some(html_str))
                        }
                    }
                }
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            Method::POST => {
                println!(
                    "fffffffffffffffffffffffffffffffffffffffffffffffff{:?}",
                    request.path()
                );
                Response::new(StatusCode::Ok, Some("from the put space".to_string()))
            }
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}

//impl Clone for WebsiteHandler {}

unsafe impl Send for WebsiteHandler {}
