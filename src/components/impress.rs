use sycamore::prelude::*;

#[component]
pub fn Impress<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        script (src="/.perseus/static/js/impress.js")
        script { "impress().init()" }
    }
}