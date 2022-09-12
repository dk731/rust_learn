use serde::{Deserialize, Serialize};
use std::str;

enum Methode {
    GET,
    POST,
    DELETE,
    //...
}

struct Body {
    raw: String,
    parsed: Option<i32>,
}

pub struct Response {}

pub struct Request<'a> {
    methode: Methode,
    request_path: &'a str,
    body: Option<String>,
    raw: String,
}

impl<'a> Request<'a> {
    pub fn new(req_buf: [u8; 1024]) -> Option<Self> {
        let raw_req = str::from_utf8(&req_buf).unwrap().to_string();

        Some(Request {
            methode: Methode::GET,
            request_path: &raw_req[1..10],
            body: None,
            raw: raw_req.to_string(),
        })
    }

    pub fn methode(&self) -> &Methode {
        &self.methode
    }

    pub fn path(&self) -> &str {
        self.request_path
    }
}
