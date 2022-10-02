use diesel::{r2d2, r2d2::ConnectionManager, Queryable, SqliteConnection};
use serde::{Deserialize, Serialize};

pub struct AppState {
    pub app_name: String,
    pub pool: Pool,
}

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Nft {
    pub id: i32,
    pub sender: String,
    pub receiver: String,
    pub gas_price: i32,
    pub tx_value: i32,
    pub tx_time: String,
    pub block_num: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NftJson {
    pub name: String,
    pub address: String,
}
