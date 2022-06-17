use common::granular_synthesizer::GranularSynthesizer;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream,
};
use log::*;
use std::sync::Arc;
use wasm_bindgen::{prelude::*, JsCast};

pub mod components;
pub mod state;
pub mod audio;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode, so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // enables using info!() macros
    wasm_logger::init(wasm_logger::Config::default());

    // start ui
    let app_div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("#app")
        .unwrap()
        .unwrap();
    yew::start_app_in_element::<components::app::App>(app_div);

    Ok(())
}
