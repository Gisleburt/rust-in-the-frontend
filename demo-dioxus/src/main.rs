#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(App);
}

fn Counter(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    render!(
        p { "{count}" }
        button { onclick: move |_| count += 1, "+1" }
        button { onclick: move |_| count -= 1, "-1" }
    )
}

fn App(cx: Scope) -> Element {
    render!(
        h1 { "Counter Example - Dioxus" }
        Counter { }
    )
}
