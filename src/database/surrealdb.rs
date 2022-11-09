// 可能使用这个新玩意, 但不确定, 先放在这里

use anyhow::Ok;
use serde_json::map;
use surrealdb::{
    channel::{new, Receiver, Sender},
    sql, Datastore, Session,
};

#[allow(dead_code)]
// 利用websocket连接到surrealdb
pub struct Surreal {
    data_store: Datastore,
    session: Session,
}

impl Surreal {}
