use anyhow::{Ok, Result};
use proc_qq::{event, module, MessageEvent, Module};

use crate::modules::types::{ID, MENU, NAME};

const ID: ID = "info";
const NAME: NAME = "osu! info";
const MENU: MENU = "osu! info";

pub fn module() -> Module {
    module!(ID, NAME, info)
}

#[event]
async fn info(event: &MessageEvent) -> anyhow::Result<bool> {
    Ok(true)
}