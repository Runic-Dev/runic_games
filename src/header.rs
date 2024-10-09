use leptos::{component, view, IntoView};

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="bg-slate-400 flex justify-center p-2 font-heading">Runic Games</header>
    }
}
