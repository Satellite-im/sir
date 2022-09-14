#![allow(non_snake_case)]

use dioxus::prelude::*;
use sir::{css, global_css, AppStyle};

fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx!(AppStyle {}, Counter {}))
}

fn Counter(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

    global_css!(
        "
        body {
            background: slategray;
        }
    "
    );

    let container = css!(
        "
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 4px;
    "
    );

    let title = css!("color: white");

    let button = css!(
        "
        border: none;
        padding: 8px 16px;
        border-radius: 4px;
        
        background: deepskyblue;
        transition: background 0.2s ease-out;

        &:hover {
            background: aquamarine;
        }
    "
    );

    cx.render(rsx!(
        div {
            class: "{container}",
            h1 { class: "{title}", "Counter: {count}" }
            button { class: "{button}", onclick: move |_| count += 1, "Increment" }
            button { class: "{button}", onclick: move |_| count -= 1, "Decrement" }
        }
    ))
}
