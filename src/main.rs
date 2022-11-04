use proc_qq::re_exports::ricq::version::MACOS;
use proc_qq::*;

use mobot::init_tracing_subscriber;
use mobot::modules;
use mobot::on_result;
#[allow(unused_imports)]
use mobot::server;

#[tokio::main]
async fn main() {
    init_tracing_subscriber();
    ClientBuilder::new()
        .authentication(Authentication::QRCode)
        .show_rq(ShowQR::OpenBySystem)
        .device(DeviceSource::JsonFile("device.json".to_owned()))
        .version(&MACOS)
        .modules(modules::all_modules())
        .result_handlers(vec![on_result {}.into()])
        .build()
        .await
        .unwrap()
        .start()
        .await
        .unwrap()
        .unwrap();
}