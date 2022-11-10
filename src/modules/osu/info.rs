use anyhow::{Ok, Result};
use proc_qq::{event, module, MessageEvent, Module};

use crate::modules::types::{COMMAND, MENU, NAME};

const COMMAND: COMMAND = "info";
const NAME: NAME = "osu! info";
const MENU: MENU = "osu! info";

pub fn module() -> Module {
    module!(COMMAND, NAME, info)
}

#[event]
async fn info(event: &MessageEvent) -> anyhow::Result<bool> {
    Ok(true)
}
