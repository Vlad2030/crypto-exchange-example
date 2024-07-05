use reqwest::Error;

use crate::core::http::client::{HttpClient, HttpMethods};


#[derive(serde::Deserialize, Debug, Clone)]
pub struct Ping {
    pub ping: bool,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ServerTime {
    pub server_time: i32,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ExchangeInfo {
    pub exchange_filters: Vec<serde_json::Value>,
    pub rate_limits: Vec<serde_json::Value>,
    pub server_time: i32,
    pub symbols: Vec<SymbolInfo>,
    pub timezone: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct SymbolInfo {
    pub base_asset: String,
    pub base_asset_precision: i32,
    pub base_commission_precision: i32,
    pub base_size_precision: String,
    pub filters: Vec<String>,
    pub full_name: String,
    pub is_margin_trading_allowed: bool,
    pub is_spot_trading_allowed: bool,
    pub maker_commission: String,
    pub max_quote_amount: String,
    pub max_quote_amount_market: String,
    pub order_types: Vec<String>,
    pub permissions: Vec<String>,
    pub quote_amount_precision: String,
    pub quote_amount_precision_market: String,
    pub quote_asset: String,
    pub quote_asset_precision: i32,
    pub quote_commission_precision: i32,
    pub quote_precision: i32,
    pub status: String,
    pub symbol: String,
    pub taker_commission: String,
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

    pub async fn exchange_info(&mut self) -> Result<ExchangeInfo, Error> {
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
