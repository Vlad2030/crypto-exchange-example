use std::collections::HashMap;
use tokio::time;
use log::{error, info, warn};
use http::{HeaderMap, HeaderName, HeaderValue};

use reqwest::{Client, Method, Error, Response};
use crate::core::http::rate_limits::RateLimits;
use crate::utils::url;

pub enum HttpMethods {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

impl HttpMethods {
    pub fn as_string(&self) -> String {
        match self {
            Self::Get => return "GET".into(),
            Self::Head => return "HEAD".into(),
            Self::Post => return "POST".into(),
            Self::Put => return "PUT".into(),
            Self::Delete => return "DELETE".into(),
            Self::Connect => return "CONNECT".into(),
            Self::Options => return "OPTIONS".into(),
            Self::Trace => return "TRACE".into(),
            Self::Patch => return "PATCH".into(),
        }
    }

    pub fn as_vec() -> Vec<String> {
        return vec![
            Self::Get.as_string(),
            Self::Head.as_string(),
            Self::Post.as_string(),
            Self::Put.as_string(),
            Self::Delete.as_string(),
            Self::Connect.as_string(),
            Self::Options.as_string(),
            Self::Trace.as_string(),
            Self::Patch.as_string(),
        ];
    }

    pub fn as_method(&self) -> Method {
        match self {
            Self::Get => return Method::GET,
            Self::Head => return Method::HEAD,
            Self::Post => return Method::POST,
            Self::Put => return Method::PUT,
            Self::Delete => return Method::DELETE,
            Self::Connect => return Method::CONNECT,
            Self::Options => return Method::OPTIONS,
            Self::Trace => return Method::TRACE,
            Self::Patch => return Method::PATCH,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status_code: i32,
    pub json: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct HttpClient {
    client: Client,
    base_url: String,
    headers: HeaderMap,
    allowed_methods: Vec<String>,
    rate_limits: RateLimits,
    enable_logging: bool,
}

impl HttpClient {
    pub fn new(
        base_url: String,
        headers: Option<HashMap<String, String>>,
        allowed_methods: Option<Vec<String>>,
        rate_limits: Option<i32>,
        enable_logging: Option<bool>,
    ) -> Self {
        let mut _headers: HeaderMap = HeaderMap::new();

        ///////// I WILL FIX IT LATER! //////////

        // if headers.is_some() {
        //     for (name, value) in headers.unwrap().iter() {
        //         // _headers.insert(key, val)
        //         _headers.insert(
        //             name.as_str(),
        //             HeaderValue::from_str(value.as_str()).unwrap(),
        //         );
        //     }
        // }

        return Self {
            client: Client::new(),
            base_url,
            headers: _headers,
            allowed_methods: allowed_methods.unwrap_or(HttpMethods::as_vec()),
            rate_limits: RateLimits::new(rate_limits.unwrap_or(1337_420_666)),
            enable_logging: enable_logging.unwrap_or(false),
        };
    }

    pub async fn request(
        &mut self,
        method: HttpMethods,
        endpoint: String,
        params: Option<HashMap<String, String>>,
        json: Option<HashMap<(String, i64), (String, i64)>>,
    ) -> Result<HttpResponse, Error> {
        let url: String = url::build(self.base_url.clone(), Some(endpoint), params);

        if !self.allowed_methods.contains(&method.as_string()) {
            let error_text: String =
                format!("Allowed only {} methods!", self.allowed_methods.join(", "));

            if self.enable_logging {
                error!("{}", error_text)
            }

            panic!("{}", error_text);
        }

        if self.rate_limits.is_limited() {
            time::sleep(time::Duration::new(1, 0)).await;

            if self.enable_logging {
                warn!("Reached maximum of rps amount, sleep for 1 secs...")
            }
        }

        if self.enable_logging {
            info!(
                "{}\t| {}\t| RPS: {}",
                method.as_string(),
                url,
                self.rate_limits.amount()
            )
        }

        let raw_request: Response = self
            .client
            .request(method.as_method(), url)
            // .headers(self.headers.clone())
            .json(&json)
            .send()
            .await?;

        self.rate_limits.new_request();

        return Ok(
            HttpResponse{
                status_code: raw_request.status().as_u16() as i32,
                json: raw_request.json().await?,
            }
        );
    }
}
