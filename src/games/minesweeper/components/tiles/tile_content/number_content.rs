use leptos::{component, view, IntoView};

use super::get_generic_classes;

#[component]
pub fn NumberContent(number: usize) -> impl IntoView {
    let mut classes = get_generic_classes();
    let classes_for_number = match number {
        1 => "bg-themeDarkGreen text-cyan-500",
        2 => "bg-themeDarkGreen text-lime-500",
        3 => "bg-themeDarkGreen text-fuchsia-500",
        4 => "bg-themeDarkGreen text-pink-500",
        5 => "bg-themeDarkGreen text-rose-500",
        _ => "bg-themeDarkGreen",
    };
    classes.push(classes_for_number);
    let classes = classes.join(" ");

    if number > 0 {
        view! {
            <span class=classes>{number}</span>
        }
    } else {
        view! {
            <span class=classes></span>
        }
    }
}
