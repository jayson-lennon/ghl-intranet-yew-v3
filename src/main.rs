// src/main.rs

mod app;
mod components;
mod time;
mod views;

use app::App;

fn main() {
    
    // initialize the wasm_logger
    wasm_logger::init(wasm_logger::Config::default());

    // render the main App
    yew::Renderer::<App>::new().render();
}
