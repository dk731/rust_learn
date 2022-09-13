use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash, os::linux::raw, str};

#[derive(Debug)]
pub enum Methode {
    GET,
    POST,
    DELETE,
    //...
    INVALID,
}

struct Body {
    raw: String,
    parsed: Option<i32>,
}

pub struct Response<'a> {
    status_code: u32,
    reason: String,
    pub headers: HashMap<String, String>,
    body: Option<&'a str>,
}

pub struct Request<'a> {
    methode: Methode,
    protocol: &'a str,
    request_path: &'a str,
    body: Option<Body>,
    raw: &'a str,
}

impl Methode {
    pub fn new(name: &str) -> Self {
        match name {
            "GET" => Methode::GET,
            "POST" => Methode::POST,
            "DELETE" => Methode::DELETE,
            _ => Methode::INVALID,
        }
    }
}

impl<'a> Request<'a> {
    pub fn new(req_buf: &'a [u8; 1024]) -> Result<Self, String> {
        let raw = str::from_utf8(req_buf).unwrap();
        let mut lines = raw.lines();

        let mut req_line = lines.next().unwrap().split(" ");

        let methode = Methode::new(req_line.next().unwrap());
        let request_path = req_line.next().unwrap();
        let protocol = req_line.next().unwrap();

        Ok(Request {
            methode,
            protocol,
            request_path,
            body: None,
            raw,
        })
    }

    pub fn methode(&self) -> &Methode {
        &self.methode
    }

    pub fn protocol(&self) -> &str {
        self.protocol
    }

    pub fn request_path(&self) -> &str {
        self.request_path
    }
}

impl<'a> Response<'a> {
    pub fn new() -> Self {
        Response {
            status_code: 200,
            reason: "OK".to_string(),
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn set_body(&mut self, new_body: &'a str) {
        self.headers
            .insert("Content-Length".to_string(), new_body.len().to_string());
        self.body = Some(new_body);
    }

    pub fn result(&self) -> String {
        let mut res = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.reason);

        for (k, v) in &self.headers {
            res.push_str(&format!("{}: {}\r\n", k, v));
        }
        res.push_str("\r\n");

        if let Some(body) = self.body {
            res.push_str(body);
            res.push_str("\r\n");
        }

        res
    }
}
