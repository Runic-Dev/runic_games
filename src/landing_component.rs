use leptos::{component, view, IntoView};

#[component]
pub fn LandingComponent() -> impl IntoView {
    let button_classes = "p-2 m-2 bg-red-200 rounded text-center text-lg";
    view! {
        <div class="h-auto w-full flex-1 flex flex-col justify-center items-center">
            <ul class="h-content w-full flex flex-col justify-between items-center">
                <li><button class=button_classes>Minesweeper</button></li>
                <li class="m-2">More coming soon...</li>
            </ul>
        </div>
    }
}
