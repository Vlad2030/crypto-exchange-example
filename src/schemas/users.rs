use serde;

use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: Option<String>,
    pub email: Option<String>,
    pub number: Option<String>,
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub can_deposit: bool,
    pub account_type: Vec<String>,
    pub assets: Vec<Uuid>,
    pub orders: Vec<Uuid>,
    pub tx_history: Vec<Uuid>,
    pub created_at: u32,
    pub updated_at: u32,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct UserAssets {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub user_id: Uuid,
    pub free: f64,
    pub locked: f64,
    pub created_at: u32,
    pub updated_at: u32,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct UserOrders {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub user_id: Uuid,
    pub price: f64,
    pub stop_price: f64,
    pub orig_qty: f64,
    pub executed_qty: f64,
    pub cummulative_quote_qty: f64,
    pub status: String,
    pub time_in_force: String,
    pub otype: String,
    pub side: String,
    pub is_working: bool,
    pub created_at: u32,
    pub updated_at: u32,
}
