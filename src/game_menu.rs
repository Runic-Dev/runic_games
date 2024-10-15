use leptos::{component, view, IntoView};
use leptos_router::use_navigate;

#[component]
pub fn GameMenu() -> impl IntoView {
    let button_classes = "collapse-title text-xl font-medium bg-themeGreen text-white";
    let navigate = use_navigate();
    view! {
        <div class="w-2/3">
            <div class="m-2 collapse collapse-arrow bg-base-200">
                <input type="radio" name="accordian" />
                <div class=button_classes>Minesweeper</div>
                <div class="collapse-content flex flex-col justify-center align-items">
                    <p class="pt-4">"A modern version of the classic Minesweeper game"</p>
                    <button on:click=move |_| { navigate("/minesweeper", Default::default()) } class="btn bg-themeGreen text-white mt-4">"Let's go!"</button>
                </div>
            </div>
        </div>
    }
}
