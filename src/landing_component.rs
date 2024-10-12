use leptos::{component, view, IntoView};

use crate::game_menu::GameMenu;

#[component]
pub fn LandingComponent() -> impl IntoView {
    let button_classes =
        "btn w-content p-2 m-2 bg-theme-green rounded text-center text-white border-theme-dark-green";
    view! {
        <div class="h-auto w-full bg-theme-grey flex-1 flex flex-col justify-center items-center">
            <GameMenu />
            // <ul class="h-content w-full flex flex-col justify-between items-center">
            //     <li><button class=button_classes data-theme="none">Minesweeper</button></li>
            //     <li class="m-2">More coming soon...</li>
            // </ul>
        </div>
    }
}
