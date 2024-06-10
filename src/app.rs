use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div id="main-window" class="window">
            <div class="title-bar">
                <div class="title-bar-text">"Sample Overlay"</div>
                <div class="title-bar-controls">
                <button aria-label="Minimize"></button>
                <button aria-label="Maximize"></button>
                <button aria-label="Close"></button>
                </div>
            </div>
            <div class="window-body">
                <span class="top-row">
                    <span class="partial-row">
                        <div class="overlay-box">A</div>
                        <div class="overlay-box">B</div>
                        <div class="overlay-box">C</div>
                    </span>
                    <span class="partial-row">
                        <div class="overlay-box">A</div>
                        <div class="overlay-box">B</div>
                    </span>
                </span>
            </div>
        </div>
    }
}
