use serde;

use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Order {
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub asset_id: Uuid,
}
