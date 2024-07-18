use std::{error::Error, time::Duration};

use serde::{Deserialize, Serialize};

use crate::model::{error::ResponseError, token::Token};

const BASE_URL: &'static str = "https://api.reyden-x.com/v1";

#[derive(Serialize, Deserialize, Debug)]
struct Detail {
    msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DetailedError {
    detail: Vec<Detail>,
}

pub trait Requests {
    fn request(
        &self,
        method: reqwest::Method,
        path: &str,
        payload: Option<String>,
    ) -> Result<String, Box<dyn Error>>;

    fn get(&self, path: &str) -> Result<String, Box<dyn Error>>;

    fn post(&self, path: &str, payload: String) -> Result<String, Box<dyn Error>>;

    fn delete(&self, path: &str) -> Result<String, Box<dyn Error>>;

    fn patch(&self, path: &str, payload: Option<String>) -> Result<String, Box<dyn Error>>;
}

pub trait Auth<T> {
    fn auth(&mut self) -> Result<&T, Box<dyn Error>>;

    fn is_authenticated(&self) -> bool;

    fn get_token(&self) -> &Token;
}

#[derive(Debug)]
pub struct Client {
    client: reqwest::blocking::Client,
    username: String,
    password: String,
    token: Token,
}

impl Client {
    pub fn new(username: String, password: String) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            username,
            password,
            token: Token {
                access_token: "".to_string(),
                expires_in: "".to_string(),
            },
        }
    }
}

impl Auth<Client> for Client {
    fn auth(&mut self) -> Result<&Client, Box<dyn Error>> {
        if self.is_authenticated() {
            return Ok(self);
        }

        let params = [("username", &self.username), ("password", &self.password)];
        let resp = self
            .client
            .post(BASE_URL.to_owned() + "/token/")
            .form(&params)
            .send()?;

        match resp.status() {
            reqwest::StatusCode::OK => {
                let text = &resp.text().unwrap();
                let token: Token = serde_json::from_str(&text)?;
                self.token = token;
                Ok(self)
            }
            reqwest::StatusCode::TOO_MANY_REQUESTS => Err(Box::new(ResponseError {
                status: resp.status(),
                message: String::from("Too many requests"),
            })),
            reqwest::StatusCode::UNPROCESSABLE_ENTITY => Err(Box::new(ResponseError {
                status: resp.status(),
                message: String::from("Validation error"),
            })),
            _ => Err(Box::new(ResponseError {
                status: reqwest::StatusCode::BAD_REQUEST,
                message: String::from("Invalid credentials"),
            })),
        }
    }

    fn is_authenticated(&self) -> bool {
        self.token.is_valid()
    }

    fn get_token(&self) -> &Token {
        &self.token
    }
}

impl Requests for Client {
    fn request(
        &self,
        method: reqwest::Method,
        path: &str,
        payload: Option<String>,
    ) -> Result<String, Box<dyn Error>> {
        let full_path = String::from(self::BASE_URL.to_string() + path);
        let cl = match method {
            reqwest::Method::POST => match payload {
                Some(data) => self
                    .client
                    .post(full_path)
                    .json(&data)
                    .header("Content-Type", "application/json"),
                None => self.client.post(full_path),
            },
            reqwest::Method::DELETE => self.client.delete(full_path),
            reqwest::Method::PATCH => self.client.patch(full_path),
            _ => self.client.get(full_path),
        };

        let resp = cl
            .header(
                "Authorization",
                format!("Bearer {}", self.get_token().access_token),
            )
            .header("Accept", "application/json")
            .timeout(Duration::new(5, 0))
            .send()?;

        match resp.status() {
            reqwest::StatusCode::OK => Ok(resp.text().unwrap()),
            reqwest::StatusCode::TOO_MANY_REQUESTS => Err(Box::new(ResponseError {
                status: resp.status(),
                message: String::from("Too many requests"),
            })),
            reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                let text = resp.text().unwrap();
                let res: DetailedError = serde_json::from_str(&text)?;
                Err(Box::new(ResponseError {
                    status: reqwest::StatusCode::UNPROCESSABLE_ENTITY,
                    message: format!("Validation error: {}", res.detail[0].msg),
                }))
            }
            reqwest::StatusCode::UNAUTHORIZED => Err(Box::new(ResponseError {
                status: resp.status(),
                message: String::from("Unauthorized"),
            })),
            _ => Err(Box::new(ResponseError {
                status: reqwest::StatusCode::BAD_REQUEST,
                message: String::from("Invalid credentials"),
            })),
        }
    }

    fn get(&self, path: &str) -> Result<String, Box<dyn Error>> {
        self.request(reqwest::Method::GET, path, None)
    }

    fn post(&self, path: &str, payload: String) -> Result<String, Box<dyn Error>> {
        self.request(reqwest::Method::POST, path, Some(payload))
    }

    fn delete(&self, path: &str) -> Result<String, Box<dyn Error>> {
        self.request(reqwest::Method::DELETE, path, None)
    }

    fn patch(&self, path: &str, payload: Option<String>) -> Result<String, Box<dyn Error>> {
        self.request(reqwest::Method::PATCH, path, payload)
    }
}
