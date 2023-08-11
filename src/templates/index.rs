use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        // Don't worry, there are much better ways of styling in Perseus!
        div(id = "impress", data-width="1024", data-height="768", data-max-scale="1", data-min-scale="0") {
            div (id = "intro", class = "step") {
                h1 { "Rust in the FrontEnd" }
                h2 { "A breif guide to making websites with Rust in 2023" }
            }

            div (id = "test", class = "step", data-x = "1000", data-rotate = "90") {
                "This is slide 2"
            }
        }

        script (src="https://cdn.jsdelivr.net/gh/impress/impress.js@2.0.0/js/impress.js")
        script { "window.impress || document.write(`<script src='https://cdn.jsdelivr.net/gh/impress/impress.js@2.0.0/js/impress.js'>\x3C/script>`);" }
        script { "impress().init()" }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Rust in the FrontEnd" }
        link (rel = "stylesheet", href = "https://cdn.jsdelivr.net/gh/impress/impress.js@2.0.0/css/impress-common.css")
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}
