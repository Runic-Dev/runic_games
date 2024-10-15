use leptos::component;
use leptos::create_rw_signal;
use leptos::provide_context;
use leptos::view;
use leptos::IntoView;
use leptos::RwSignal;

use crate::games::minesweeper::colors::BLUE;
use crate::games::minesweeper::components::main_body::main_body_component::MainBody;

use super::game_state::GameState;

#[derive(Copy, Clone)]
pub struct GameStateSetter(pub RwSignal<GameState>);

#[component]
pub fn MinesweeperGame() -> impl IntoView {
    let game_state = create_rw_signal(GameState::default());
    provide_context(GameStateSetter(game_state));
    let classes = format!(
        "h-full w-full flex justify-center items-center bg-[radial-gradient(circle,_transparent_70%,_{}_100%)]",
        BLUE
    );
    view! {
        <div class=classes>
            <div class="container mx-auto flex flex-col justify-center min-h-full">
                <MainBody />
            </div>
        </div>
    }
}
