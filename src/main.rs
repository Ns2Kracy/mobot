use proc_qq::re_exports::ricq::version::MACOS;
use proc_qq::*;

use darling_bot::modules;
use darling_bot::result_handlers;
use darling_bot::init_tracing_subscriber;


#[tokio::main]
async fn main() {
    init_tracing_subscriber();
    ClientBuilder::new()
        .authentication(Authentication::QRCode)
        .show_rq(ShowQR::OpenBySystem)
        .device(DeviceSource::JsonFile("device.json".to_owned()))
        .version(&MACOS)
        .modules(modules::all_modules())
        .result_handlers(vec![result_handlers::on_result {}.into()])
        .build()
        .await
        .unwrap()
        .start()
        .await
        .unwrap()
        .unwrap();
}


