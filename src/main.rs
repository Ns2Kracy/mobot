use mobot::config::load_config;
use mobot::database::mysql::init_mysql;
use mobot::database::redis::init_redis;
use mobot::modules;
use proc_qq::re_exports::ricq::version::MACOS;
#[allow(unused_imports)]
use proc_qq::Authentication::{QRCode, UinPassword};
use proc_qq::*;
use std::sync::Arc;
use tracing::Level;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing_subscriber();
    let config = load_config().await?;
    init_redis(&config.redis).await?;
    init_mysql(&config.mysql).await?;
    let client = ClientBuilder::new()
        .device(DeviceSource::JsonFile("device.json".to_owned()))
        .version(&MACOS)
        .priority_session("session.token")
        .authentication(UinPassword(config.account.uin, config.account.password))
        .show_slider_pop_menu_if_possible()
        // .authentication(QRCode)
        // .show_rq(ShowQR::PrintToConsole)
        .modules(modules::all_modules())
        .build()
        .await
        .unwrap();
    // 可以做一些定时任务, rq_client在一开始可能没有登录好
    let client = Arc::new(client);
    let copy = client.clone();
    tokio::spawn(async move {
        println!("{}", copy.rq_client.start_time);
    });
    run_client(client).await?;
    Ok(())
}

fn init_tracing_subscriber() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .without_time(),
        )
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_target("ricq", Level::DEBUG)
                .with_target("proc_qq", Level::DEBUG)
                .with_target("mobot", Level::DEBUG),
        )
        .init();
}
