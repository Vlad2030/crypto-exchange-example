use serde;

use uuid::Uuid;

use crate::schemas::orders::Order;


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Asset {
    pub id: Uuid,
    pub symbol: String,
    pub status: String,
    pub base_asset: String,
    pub base_asset_precision: u32,
    pub quote_asset: String,
    pub quote_precision: u32,
    pub quote_asset_precision: u32,
    pub base_commission_precision: u32,
    pub quote_commission_precision: u32,
    pub order_types: Vec<String>,
    pub quote_order_qty_market_allowed: bool,
    pub is_spot_trading_allowed: bool,
    pub is_margin_trading_allowed: bool,
    pub quote_amount_precision: f32,
    pub base_size_precision: f32,
    pub permissions: Vec<String>,
    pub filters: Vec<String>,
    pub max_quote_amount: f32,
    pub maker_commission: f32,
    pub taker_commission: f32,
    pub full_name: String,
    pub networks: Vec<Uuid>,
    pub orders: Vec<Order>,
    pub created_at: u32,
    pub updated_at: u32,
}
