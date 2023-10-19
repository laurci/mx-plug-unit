#![no_std]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use mx_plug_shared::arg;

#[arg]
pub struct PushTextArgs {
    pub topic: String,
    pub text: String,
}

#[arg]
pub struct PushBinaryArgs {
    pub topic: String,
    pub data: Vec<u8>,
}
