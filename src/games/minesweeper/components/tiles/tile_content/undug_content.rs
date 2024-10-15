use leptos::{component, view, IntoView};

use super::get_generic_classes;

#[component]
pub fn UndugTileContent(is_flagged: bool) -> impl IntoView {
    let mut classes = get_generic_classes();
    classes.extend(&[
        "bg-theme-green",
        "border-solid border-2 border-theme-dark-green",
        "text-slate-800",
        "p-1",
        "z-[9999]",
    ]);
    let classes = classes.join(" ");

    if is_flagged {
        view! {
            <div class=classes>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 116.8 122.88" fill="none">
                    <path d="M18,81.08l-5.78-56.9A4.3,4.3,0,0,1,14.39,20C41.59,2.6,54.66,9.66,66.7,16.16,76.22,21.3,84.92,26,103.75,10a4.45,4.45,0,0,1,6.2.44,4.22,4.22,0,0,1,1,2.42l5.78,56.89a4.23,4.23,0,0,1-1.38,3.57c-21.79,19.84-35,13.16-48.6,6.27C55.74,74,44.35,68.25,25.21,84.12a4.47,4.47,0,0,1-6.21-.5,4.26,4.26,0,0,1-1-2.54Z" fill="#e21b1b"/>
                    <path d="M17.89,16.71l9.88,98.6a6.89,6.89,0,1,1-13.71,1.35L4.21,18.38a10.15,10.15,0,1,1,13.68-1.67Z" fill="#1a1a1a"/>
                </svg>
            </div>
        }
    } else {
        view! {
            <div class=classes></div>
        }
    }
}
