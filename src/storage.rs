use serde;

use crate::schemas::users::{User, UserAssets, UserOrders};
use crate::schemas::assets::Asset;
use crate::schemas::networks::{Network, NetworkTransactions};


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Storage {
    pub users: Vec<User>,
    pub users_assets: Vec<UserAssets>,
    pub users_orders: Vec<UserOrders>,

    pub assets: Vec<Asset>,

    pub networks: Vec<Network>,
    pub networks_txns: Vec<NetworkTransactions>,
}

pub static mut STORAGE: Storage = Storage{
    users: Vec::new(),
    users_assets: Vec::new(),
    users_orders: Vec::new(),
    assets: Vec::new(),
    networks: Vec::new(),
    networks_txns: Vec::new(),
};
