use leptos::{component, view, IntoView};
use leptos_router::{Route, Router, Routes};

use crate::{game_menu::GameMenu, games::minesweeper::minesweeper_game::MinesweeperGame};

#[component]
pub fn LandingComponent() -> impl IntoView {
    view! {
        <div class="bg-themeGrey h-auto w-full flex-1 flex flex-col justify-center items-center">
            <Router>
                <Routes>
                    <Route path="/" view=|| view! { <GameMenu />} />
                    <Route path="/minesweeper" view=|| view! { <MinesweeperGame /> } />
                </Routes>
            </Router>
        </div>
    }
}
