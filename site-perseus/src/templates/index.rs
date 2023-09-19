use perseus::prelude::*;
use sycamore::prelude::*;
use crate::components::{step::Step, impress::Impress, code::Code};

fn index_page<G: Html>(cx: Scope) -> View<G> {
    let col_mul = 1200;
    let row_mul = 1000;
    view! { cx,
        Impress(enabled = true) {
            Step (name = "intro", x = 0 * col_mul) {
                header {
                    img (
                        src = "/.perseus/static/images/rustacean-flat-happy.png",
                        alt = "Ferris the Crab looking happy"
                    )
                    h1 { "Rust in the Frontend" }
                    h2 { "A brief guide to making websites with Rust in 2023" }
                    h2 { "Daniel Mason" }
                }
            }

            Step (name = "why", x = 1 * col_mul) {
                div( class = "center" ) {
                    img (
                        src = "/.perseus/static/images/why-jackie-chan.png",
                        alt = "Jackie Chan: Why would you do that?"
                    )
                    h2 ( style = "display: none" ) { "Why would I want to do that?" }
                }
            }

            Step (name = "why-speed-rust", x = 2 * col_mul) {
                h3 { "Rust is fast" }
                p { "Rust is a language known for its speed" }
                img (src="/.perseus/static/images/speed-elapsed-seconds.png")
            }

            Step (name = "why-speed-wasm", x = 3 * col_mul) {
                h3 { "WASM is fast" }
                p { "WASM is known to provide a secure, high performance runtime for the web" }
                img (src="/.perseus/static/images/speed-wasm.png")
            }

            Step (name = "why-rust-wasm", x = 4 * col_mul) {
                h3 { "Rust WASM ecosystem is very mature" }
                ul {
                    li {
                        "Tier 2: "
                        span (class = "code ") { "rustup add target wasm32-unknown-unknown" }
                    }
                    li {
                        "Living book, tutorials, tools: "
                        a(href = "https://rustwasm.github.io/") {"https://rustwasm.github.io/"}
                    }
                    li {
                        "Write once, run anywhere: "
                        a(href = "https://tauri.app/") {"https://tauri.app/"}
                    }
                }
            }

            Step (name = "why-rust-correctness", x = 5 * col_mul) {
                h3 { "Rust is robust, reliable and 'correct'" }
                p {"Typescript:"}
                Code(language = "typescript") {
                    "async function getUser(email: string): Promise<Result<Error, User>> { \n"
                    "  const response = await fetch(`https://example.com/${email}`);\n"
                    "  const user = await response.json()\n"
                    "  return user.email === email\n"
                    "    ? Result.ok(user)\n"
                    "    : Result.error(new Error('Incorrect user returned');\n"
                    "}\n"
                }
                p {"Rust:"}
                Code(language = "rust") {
                    "async fn getUser(email: &str) -> Result<User, GetUserError> {\n"
                    "    let user: User = get(format!(\"https://example.com/{email}\")).await?\n"
                    "        .json().await?;\n"
                    "    if user.email == email {\n"
                    "        Ok(user)\n"
                    "    } else {\n"
                    "        Err(GetUserError::IncorrectUserReturned)\n"
                    "    }\n"
                    "}\n"
                }
            }

            Step (name = "options", y = 1 * row_mul, x = 0 * col_mul) {
                h2 { "What are my options" }
            }

            Step (name = "options-yew", y = 2 * row_mul, x = 0 * col_mul) {
                header {
                    img (
                        class = "logo",
                        src = "/.perseus/static/images/logo-yew.png",
                        alt = "Sycamore Logo"
                    )
                    h3 { "Yew" }
                }
                ul {
                    li { "Yew looks a lot like React" }
                    li { "Uses a Virtual DOM" }
                    li { "Supports CSR, SSR, and SSG" }
                }
            }

            Step (name = "options-yew-demo-counter", y = 2 * row_mul, x = 1 * col_mul) {
                h3 { "Yew Counter" }
                Code(language = "rust") {
                    "#[function_component]\n"
                    "fn Counter() -> Html {\n"
                    "    let counter = use_state(|| 0);\n"
                    "    let increment = {\n"
                    "        let counter = counter.clone();\n"
                    "        move |_| { counter.set(*counter + 1); }\n"
                    "    };\n"
                    "    let decrement = {\n"
                    "        let counter = counter.clone();\n"
                    "        move |_| { counter.set(*counter - 1); }\n"
                    "    };\n"
                    "\n"
                    "    html! {\n"
                    "        <div>\n"
                    "            <p>{ *counter }</p>\n"
                    "            <button onclick={increment}>{ \"+1\" }</button>\n"
                    "            <button onclick={decrement}>{ \"-1\" }</button>\n"
                    "        </div>\n"
                    "    }\n"
                    "}\n"
                }
            }

            Step (name = "options-yew-demo-counter-usage", y = 2 * row_mul, x = 2 * col_mul) {
                h3 { "Yew Counter Usage" }
                Code (language = "rust") {
                    "#[function_component]\n"
                    "fn App() -> Html {\n"
                    "    html! {\n"
                    "        <>\n"
                    "            <h1>{\"Counter Example - Yew\"}</h1>\n"
                    "            <Counter />\n"
                    "        </>\n"
                    "    }\n"
                    "}\n"
                }
            }

            Step (name = "options-perseus", y = 3 * row_mul, x = 0 * col_mul) {
                header {
                    img (
                        class = "logo",
                        src = "/.perseus/static/images/logo-sycamore.png",
                        alt = "Sycamore Logo"
                    )
                    h3 {"Perseus (Sycamore)" }
                }
                ul {
                    li { "Perseus is to Sycamore what Next is to React" }
                    li { "Does not use Virtual DOM, uses 'fine grain reactivity'" }
                    li { "Supports CSR, SSR, and SSG" }
                }
            }

            Step (name = "options-perseus-demo-counter", y = 3 * row_mul, x = 1 * col_mul) {
                h3 { "Perseus Counter" }
                Code (language = "rust") {
                    "#[component]\n"
                    "#[auto_scope]\n"
                    "fn Counter<G: Html>(cx: Scope, state: &CounterStateRx) -> View<G> {\n"
                    "    view! { cx,\n"
                    "        div {\n"
                    "            p { (state.total.get()) }\n"
                    "            button( on:click = move |_| {state.total.set(*state.total.get() + 1)}) { \"+1\" }\n"
                    "            button( on:click = move |_| {state.total.set(*state.total.get() - 1)}) { \"-1\" }\n"
                    "        }\n"
                    "    }\n"
                    "}\n"
                }
            }

            Step (name = "options-perseus-demo-state", y = 3 * row_mul, x = 2 * col_mul) {
                h3 { "Perseus State" }
                Code (language = "rust") {
                    "#[derive(Default, Serialize, Deserialize, Clone, ReactiveState)]\n"
                    "#[rx(alias = \"CounterStateRx\")]\n"
                    "struct State {\n"
                    "    total: i32,\n"
                    "}\n"
                }
            }

            Step (name = "options-perseus-demo-counter-usage", y = 3 * row_mul, x = 3 * col_mul) {
                h3 { "Perseus Counter Usage" }
                Code (language = "rust") {
                    "#[auto_scope]\n"
                    "fn index_page<G: Html>(cx: Scope, state: &CounterStateRx) -> View<G> {\n"
                    "    view! { cx,\n"
                    "        h1 { \"Counter Example - Perseus\" }\n"
                    "        Counter(state)\n"
                    "    }\n"
                    "}\n"
                }
            }

            Step (name = "options-dioxus", y = 4 * row_mul, x = 0 * col_mul) {
                header {
                    img (
                        class = "logo",
                        src = "/.perseus/static/images/logo-dioxus.png",
                        alt = "Sycamore Logo"
                    )
                    h3 { "Dioxus" }
                }
                ul {
                    li { "Similar 'reactivity' approach to Sycamore" }
                    li { "Looks a lot like Sycamore too'" }
                    li { "Supports CSR, SSR, SSG, Desktop, and more!" }
                }
            }

            Step (name = "options-dioxus-demo-counter", y = 4 * row_mul, x = 1 * col_mul) {
                h3 { "Dioxus Counter" }
                Code (language = "rust") {
                    "fn Counter(cx: Scope) -> Element {\n"
                    "    let mut count = use_state(cx, || 0);\n"
                    "\n"
                    "    render!(\n"
                    "        p { \"{count}\" }\n"
                    "        button { onclick: move |_| count += 1, \"+1\" }\n"
                    "        button { onclick: move |_| count -= 1, \"-1\" }\n"
                    "    )\n"
                    "}\n"
                }
            }

            Step (name = "options-dioxus-demo-counter-usage", y = 4 * row_mul, x = 2 * col_mul) {
                h3 { "Dioxus Counter Usage" }
                Code (language = "rust") {
                    "fn App(cx: Scope) -> Element {\n"
                    "    render!(\n"
                    "        h1 { \"Counter Example - Dioxus\" }\n"
                    "        Counter { }\n"
                    "    )\n"
                    "}\n"
                }
            }

            Step (name = "options-comparison", y = 5 * row_mul, x = 0) {
                h3 { "Comparison" }
                ul {
                    li { "Yew is slower than React" }
                    li { "Sycamore is faster" }
                    li { "Dioxus is faster still" }
                    li { "Speed isn't everything" }
                    li { "Dioxus and Sycamore don't allow copy paste html, Yew does" }
                }
            }

            Step (name = "conclusion", y = 6 * row_mul, x = 0) {
                h2 { "Wait, should I even do this" }
                ul {
                    li { "Probably not" }
                    li { "Speed isn't everything" }
                    li { "Speed differences are small" }
                    li {
                        "But it depends"
                        ul {
                            li { "These speeds are for rendering" }
                            li { "Speed differences are much greater for raw calculations" }
                        }
                    }
                    li { "Dioxus and Sycamore don't allow copy paste html, Yew does" }
                }
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Rust in the FrontEnd" }
        link (rel = "stylesheet", href = "/.perseus/static/css/meyerweb.css")
        link (rel = "stylesheet", href = "/.perseus/static/css/impress-common.css")
        link (rel = "stylesheet", href = "/.perseus/static/css/rust-in-the-frontend.css")
        link (rel = "stylesheet", href = "/.perseus/static/css/rust-in-the-frontend.css")
        link (rel = "stylesheet", href = "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.8.0/styles/a11y-dark.min.css")
        link (rel = "apple-touch-icon",sizes = "180x180",href = "/.perseus/static/apple-touch-icon.png")
        link (rel = "icon",type = "image/png",sizes = "32x32",href = "/.perseus/static/favicon-32x32.png")
        link (rel = "icon",type = "image/png",sizes = "16x16",href = "/.perseus/static/favicon-16x16.png")
        link (rel = "manifest",href = "/.perseus/static/site.webmanifest")
        link (rel = "mask-icon",href = "/.perseus/static/safari-pinned-tab.svg",color = "#5bbad5")
        meta (name = "msapplication-TileColor",content = "#da532c")
        meta (name = "theme-color",content = "#ffffff")
        script (src = "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.8.0/highlight.min.js")
        script (src = "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.8.0/languages/rust.min.js")
        script (src = "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.8.0/languages/typescript.min.js")
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}
