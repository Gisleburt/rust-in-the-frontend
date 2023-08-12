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

        script (src="/.perseus/static/js/impress.js")
        script { "impress().init()" }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Rust in the FrontEnd" }
        link (rel = "stylesheet", href = "/.perseus/static/css/impress-common.css")
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
