use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{footer::Footer, header::Header, landing_component::LandingComponent};

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
        <div class="h-screen w-full flex flex-col">
            <Header />
            <LandingComponent />
            <Footer />
        </div>
    }
}
