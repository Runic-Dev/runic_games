mod app;
pub mod colors;
pub mod footer;
pub mod game_menu;
pub mod header;
pub mod landing_component;

use app::*;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
