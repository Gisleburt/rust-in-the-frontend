use sycamore::prelude::*;

#[derive(Prop)]
pub struct ImpressProps<'a, G: Html> {
    children: Children<'a, G>,
    #[builder(default)]
    enabled: bool,
}

#[component]
pub fn Impress<'a, G: Html>(cx: Scope<'a>, props: ImpressProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    view! { cx,
        // div (class="fallback-message") {
        //     p { "Your browser " b { "doesn't support the features required" } "by impress.js, so you are presented with a simplified version of this presentation." }
        //     p { "For the best experience please use the latest Chrome, Safari or Firefox browser." }
        // }

        div (
            id = "impress",
            data-transition-duration = 1000,
            data-width = 1024,
            data-height = 768,
            data-max-scale = 3,
            data-min-scale = 0,
            data-perspective = 1000
        ) {
            (children)
        }
        (if props.enabled {
            view! { cx,
                script (src=".perseus/static/js/impress.js")
                script { "window.impress || document.write(`<script src='https://cdn.jsdelivr.net/gh/impress/impress.js@2.0.0/js/impress.js'></script>`);" }
                script { "impress().init()" }
            }
        } else {
            view! { cx, } // Now you don't
        })
    }
}
