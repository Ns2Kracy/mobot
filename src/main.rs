use mobot::config::load_config;
use mobot::database::mysql::init_mysql;
use mobot::modules;
use proc_qq::re_exports::ricq::version::MACOS;
use proc_qq::Authentication::UinPassword;
use proc_qq::*;
use std::sync::Arc;
use tracing::Level;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	init_tracing_subscriber();
	let config = load_config().await?;
	init_mysql(&config.mysql).await?;

	tracing::info!("Starting Mobot Server...");

	let client = ClientBuilder::new()
		.device(DeviceSource::JsonFile("device.json".to_owned()))
		.version(&MACOS)
		.priority_session("session.token")
		.authentication(UinPassword(config.account.uin, config.account.password))
		.show_slider_pop_menu_if_possible()
		.modules(modules::all_modules())
		.build()
		.await
		.unwrap();
	// let mobot = tokio::fs::read_to_string("src\\mobot.txt").await?;
	// tracing::info!(mobot);
	let client = Arc::new(client);
	let copy = client.clone();
	tokio::spawn(async move {
		println!("{}", copy.rq_client.start_time);
	});
	run_client(client).await?;
	Ok(())
}

fn init_tracing_subscriber() {
	let mut guards = Vec::new();
	let file_appender = tracing_appender::rolling::never("./logs", "mobot.log");
	let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
	guards.push(guard);
	let bot_layer = tracing_subscriber::filter::Targets::new()
		.with_target("ricq", Level::INFO)
		.with_target("proc_qq", Level::DEBUG)
		.with_target("mobot", Level::DEBUG);
	let file_layer =
		tracing_subscriber::fmt::layer().pretty().with_ansi(false).with_writer(non_blocking);
	let console_layer = tracing_subscriber::fmt::layer()
		.pretty()
		.without_time()
		.with_ansi(true)
		.with_target(true);
	tracing_subscriber::registry().with(file_layer).with(bot_layer).with(console_layer).init();
}
