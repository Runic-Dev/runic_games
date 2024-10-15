use leptos::view;
use leptos::{component, IntoView};

use crate::games::minesweeper::components::tiles::tile_content::get_generic_classes;

#[component]
pub fn BombImg() -> impl IntoView {
    let mut classes = get_generic_classes();
    classes.extend(&["bg-red-400"]);
    let classes = classes.join(" ");

    view! {
        <div class=classes>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none">
                <path d="M17 7L15 9" stroke="#FFFFFF" stroke-width="1.5" stroke-linecap="round"/>
                <path d="M19.5 7.5L20.5 8" stroke="#FFFFFF" stroke-width="1.5" stroke-linecap="round"/>
                <path d="M16 3.5L16.5 4.5" stroke="#FFFFFF" stroke-width="1.5" stroke-linecap="round"/>
                <path d="M19 5L20 4" stroke="#FFFFFF" stroke-width="1.5" stroke-linecap="round"/>
                <path d="M5.75 8.00337C6.85315 7.36523 8.13392 7 9.5 7C13.6421 7 17 10.3579 17 14.5C17 18.6421 13.6421 22 9.5 22C5.35786 22 2 18.6421 2 14.5C2 13.1339 2.36523 11.8532 3.00337 10.75" stroke="#FFFFFF" stroke-width="1.5" stroke-linecap="round"/>
            </svg>
        </div>
    }
}
