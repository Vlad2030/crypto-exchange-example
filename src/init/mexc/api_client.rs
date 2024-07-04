use reqwest::{Client, Method, Error, Response};

use crate::core::http::client::{HttpClient, HttpMethods, HttpResponse};


#[derive(Debug, Clone)]
pub struct Ping {
    pub ping: bool,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ServerTime {
    pub server_time: i32,
}



#[derive(Debug, Clone)]
pub struct MexcClient {
    api_key: Option<String>,
    secret_key: Option<String>,
    enable_logging: Option<bool>,
    _http_client: HttpClient,
}

impl MexcClient {
    pub fn new(
            api_key: Option<String>,
            secret_key: Option<String>,
            enable_logging: Option<bool>,
    ) -> Self {
        let _http_client: HttpClient = HttpClient::new(
            "https://api.mexc.com".into(),
            None,
            Some(
                vec![
                    HttpMethods::Get.as_string(),
                    HttpMethods::Post.as_string(),
                    HttpMethods::Delete.as_string(),
                ]
            ),
            Some(20),
            enable_logging,
        );

        return Self {api_key, secret_key, enable_logging, _http_client};
    }

    pub async fn ping(&mut self) -> Result<Ping, Error> {
        let mut _ping: Ping = Ping{ping: true};

        let request = self
            ._http_client
            .request(
                HttpMethods::Get,
                "/api/v3/ping".into(),
                None,
                None,
            )
            .await?;

        match request.status_code {
            200 => return Ok(_ping),
            failed => {
                _ping.ping = false;
                return Ok(_ping);
            },
        }
    }

    pub async fn server_time(&mut self) -> Result<ServerTime, Error> {
        let request = self
            ._http_client
            .request(
                HttpMethods::Get,
                "/api/v3/time".into(),
                None,
                None,
            )
            .await?;

        return Ok(serde_json::from_value(request.json).unwrap())
    }

    pub async fn exchange_info(&mut self) -> Result<ServerTime, Error> {
        let request = self
            ._http_client
            .request(
                HttpMethods::Get,
                "/api/v3/exchangeInfo".into(),
                None,
                None,
            )
            .await?;

        return Ok(serde_json::from_value(request.json).unwrap())
    }
}
