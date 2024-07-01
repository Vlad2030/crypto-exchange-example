use serde;

use uuid::Uuid;


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Network {
    pub id: Uuid,
    pub coin: String,
    pub deposit_desc: Option<String>,
    pub deposit_enable: bool,
    pub min_confirm: u32,
    pub name: String,
    pub network: String,
    pub withdraw_enable: bool,
    pub withdraw_fee: f64,
    pub withdraw_integer_multiple: Option<i32>,
    pub withdraw_max: f64,
    pub withdraw_min: f64,
    pub same_address: bool,
    pub contract: String,
    pub withdraw_tips: String,
    pub deposit_tips: String,
    pub created_at: u32,
    pub updated_at: u32,
}


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct NetworkTransactions {
    pub id: Uuid,
    pub network_id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub status: u32,
    pub address: String,
    pub memo: Option<String>,
    pub tx_id: String,
    pub unlock_confirm: u32,
    pub confirm_times: u32,
    pub created_at: u32,
    pub updated_at: u32,
}
