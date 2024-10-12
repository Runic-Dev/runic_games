use leptos::{component, view, IntoView};

#[component]
pub fn GameMenu() -> impl IntoView {
    let button_classes = "collapse-title text-xl font-medium bg-theme-green text-white";
    view! {
        <div class="w-2/3">
            <div class="m-2 collapse collapse-arrow bg-base-200">
                <input type="radio" name="accordian" />
                <div class=button_classes>Minesweeper</div>
                <div class="collapse-content flex flex-col justify-center align-items">
                    <p class="pt-4">"A modern version of the classic Minesweeper game"</p>
                    <button class="btn bg-theme-green text-white mt-4">"Let's go!"</button>
                </div>
            </div>
            // <div class="m-2 collapse collapse-arrow bg-base-200">
            //     <input type="radio" name="accordian" />
            //     <div class=button_classes>Another Game</div>
            //     <div class="collapse-content">
            //         <p class="p-2"></p>
            //     </div>
            // </div>
        </div>
    }
}
