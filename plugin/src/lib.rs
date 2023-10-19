use std::env;

use mx_plug_core::{PluginContext, Void};
use shared::{PushBinaryArgs, PushTextArgs};
use unit_crossbar_client::Crossbar;

mx_plug_core::plugin! {
    name = "unit",
    fns = [push_text, push_binary]
}

fn get_crossbar_endpoint() -> String {
    env::var("UNIT_CROSSBAR_ENDPOINT").unwrap()
}

fn get_crossbar_key() -> String {
    env::var("UNIT_CROSSBAR_KEY").unwrap()
}

fn push_text(_: &PluginContext, args: PushTextArgs) -> Void {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let mut crossbar = Crossbar::new(get_crossbar_endpoint(), get_crossbar_key())
            .await
            .unwrap();
        crossbar.push_text(args.topic, args.text).await.unwrap();
    });

    return Void {};
}

fn push_binary(_: &PluginContext, args: PushBinaryArgs) -> Void {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let mut crossbar = Crossbar::new(get_crossbar_endpoint(), get_crossbar_key())
            .await
            .unwrap();
        crossbar.push_binary(args.topic, args.data).await.unwrap();
    });
    return Void {};
}
