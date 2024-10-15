use crate::games::minesweeper::components::tiles::tile_content::bomb_content::BombImg;
use crate::games::minesweeper::components::tiles::tile_content::number_content::NumberContent;
use crate::games::minesweeper::components::tiles::tile_content::undug_content::UndugTileContent;
use crate::games::minesweeper::game_state::PlayState;
use crate::games::minesweeper::{
    minesweeper_game::GameStateSetter,
    tile_state::{TileState, TileType},
};
use leptos::logging::log;
use leptos::*;

#[component]
pub fn TileSpace(
    row: usize,
    col: usize,
    #[prop(into)] cell_data: Memo<TileState>,
    on_click: impl Fn((usize, usize)) + 'static,
    on_rmb_click: impl Fn((usize, usize), (i32, i32)) + 'static,
) -> impl IntoView {
    let game_state = use_context::<GameStateSetter>().unwrap().0;
    let lmb_click_handler = move |_| {
        log!("Clicked");
        if !cell_data.get().is_dug {
            on_click((row, col));
        }
    };

    let content = move || {
        let cell = cell_data.get();
        match cell.tile_type {
            TileType::Number { local_mines } if cell.is_dug => {
                view! {
                    <NumberContent number=local_mines />
                }
            }
            TileType::Bomb if cell.is_dug => {
                view! {
                    <BombImg />
                }
            }
            _ => {
                view! {
                    <UndugTileContent is_flagged=cell.is_flagged />
                }
            }
        }
    };

    let rmb_click_handler = move |mouse_event: leptos::ev::MouseEvent| {
        mouse_event.prevent_default();
        if let PlayState::InProgress { mines_left: _ } = game_state.get().play_state {
            if !cell_data.get().is_dug {
                on_rmb_click((row, col), (mouse_event.client_x(), mouse_event.client_y()));
            }
        }
    };

    view! {
        <div on:click=lmb_click_handler on:contextmenu=rmb_click_handler>{ move || { content } }</div>
    }
}
