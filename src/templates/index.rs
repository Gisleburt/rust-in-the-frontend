use perseus::prelude::*;
use sycamore::prelude::*;
use crate::components::{step::Step, impress::Impress};

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Impress(enabled = true) {
            Step (name = "intro") {
                h1 { "Rust in the FrontEnd" }
                h2 { "A brief guide to making websites with Rust in 2023" }
            }

            Step (name = "why", x = 1000, rotate = 90) {
                h2 { "Why would you want to do this" }
            }

            Step (name = "why-speed-rust", x = 500, rotate = 90) {
                h2 { "Rust is fast" }
                p { "Rust is a language known for its speed" }
                img (src="/.perseus/static/images/speed-elapsed-seconds.png")
            }

            Step (name = "why-speed-wasm", x = 500, y = 500, rotate = 180) {
                h2 { "WASM is fast" }
                p { "WASM is known to provide a secure, high performance runtime for the web" }
                img (src="/.perseus/static/images/speed-wasm.png")
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Rust in the FrontEnd" }
        link (rel = "stylesheet", href = "/.perseus/static/css/impress-common.css")
        link (rel = "stylesheet", href = "/.perseus/static/css/impress-demo.css")
        link(rel="apple-touch-icon",sizes="180x180",href="/.perseus/static/apple-touch-icon.png")
        link(rel="icon",type="image/png",sizes="32x32",href="/.perseus/static/favicon-32x32.png")
        link(rel="icon",type="image/png",sizes="16x16",href="/.perseus/static/favicon-16x16.png")
        link(rel="manifest",href="/.perseus/static/site.webmanifest")
        link(rel="mask-icon",href="/.perseus/static/safari-pinned-tab.svg",color="#5bbad5")
        meta(name="msapplication-TileColor",content="#da532c")
        meta(name="theme-color",content="#ffffff")
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}
