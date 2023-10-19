#![no_std]

use alloc::{string::String, vec::Vec};
use mx_plug_bridge::Void;
use shared::{PushBinaryArgs, PushTextArgs};

extern crate alloc;

pub fn push_text(topic: String, text: String) {
    let _: Void =
        mx_plug_bridge::call_plugin_fn("unit", "push_text", &PushTextArgs { topic, text });
}

pub fn push_binary(topic: String, data: Vec<u8>) {
    let _: Void =
        mx_plug_bridge::call_plugin_fn("unit", "push_binary", &PushBinaryArgs { topic, data });
}

