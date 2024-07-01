use http::{HeaderMap, HeaderName, HeaderValue};
use log::{error, info};
use std::collections::HashMap;

use reqwest::{Client, Method, Error, Response};

use crate::core::http::rate_limits::RateLimits;
use crate::utils::url::build_url;

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
            Self::Get => return "GET".to_string(),
            Self::Head => return "HEAD".to_string(),
            Self::Post => return "POST".to_string(),
            Self::Put => return "PUT".to_string(),
            Self::Delete => return "DELETE".to_string(),
            Self::Connect => return "CONNECT".to_string(),
            Self::Options => return "OPTIONS".to_string(),
            Self::Trace => return "TRACE".to_string(),
            Self::Patch => return "PATCH".to_string(),
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
        &self,
        base_url: String,
        headers: Option<HashMap<String, String>>,
        allowed_methods: Option<Vec<String>>,
        rate_limits: Option<i32>,
        enable_logging: Option<bool>,
    ) -> Self {
        let mut _headers: HeaderMap = HeaderMap::new();

        if headers.is_some() {
            for (name, value) in headers.unwrap().iter() {
                // _headers.insert(key, val)
                _headers.insert(
                    name.as_str(),
                    HeaderValue::from_str(value.as_str()).unwrap(),
                );
            }
        }

        return Self {
            client: Client::new(),
            base_url,
            headers: _headers,
            allowed_methods: allowed_methods.unwrap_or(HttpMethods::as_vec()),
            rate_limits: RateLimits::new(rate_limits.unwrap_or(1337_420_666)),
            enable_logging: enable_logging.unwrap_or(false),
        };
    }

    async fn request(
        &self,
        method: HttpMethods,
        endpoint: String,
        params: Option<HashMap<String, String>>,
        json: Option<HashMap<(String, i64), (String, i64)>>,
    ) -> Result<Response, Error> {
        let url: String = build_url(self.base_url.clone(), Some(endpoint), params);

        if !self.allowed_methods.contains(&method.as_string()) {
            let error_text: String =
                format!("Allowed only {} methods!", self.allowed_methods.join(", "));

            if self.enable_logging {
                error!("{}", error_text)
            }

            panic!("{}", error_text);
        }

        if self.enable_logging {
            info!(
                "{}\t| {}\t| RPS: {}",
                method.as_string(),
                url,
                self.rate_limits.amount()
            )
        }

        let raw_request = self
            .client
            .request(method.as_method(), url)
            .headers(self.headers.clone())
            .json(&json)
            .send()
            .await?;

        return Ok(raw_request);
    }
}
