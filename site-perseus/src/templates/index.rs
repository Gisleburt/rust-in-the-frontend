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
                    li {
                        "Official Rust WASM Working Group: "
                        a(href = "https://www.rust-lang.org/governance/wgs/wasm") {"https://www.rust-lang.org/governance/wgs/wasm"}
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
                
                table {
                    thead {
                        tr {
                            th { "Name" }
                            th { "dioxus" br { } "v0.4.0" }
                            th { "sycamore" br { } "v0.8.0" }
                            th { "react" br { } "v18.2.0" }
                            th { "yew" br { } "v0.20.0" }
                        }
                    }
                    tbody {
                        tr {
                            th {"create rows" br { } "creating 1,000 rows (5 warmup runs)."}
                            td { span (class="mean") {"39.7"} br {} span (class="deviation") {"0.5"} span (class="factor") {"(1.08)"}}
                            td { span (class="mean") {"45.2"} br {} span (class="deviation") {"0.4"} span (class="factor") {"(1.23)"}}
                            td { span (class="mean") {"45.6"} br {} span (class="deviation") {"0.4"} span (class="factor") {"(1.24)"}}
                            td { span (class="mean") {"69.4"} br {} span (class="deviation") {"0.5"} span (class="factor") {"(1.88)"}}
                        }
                        tr {
                            th {"replace all rows" br { } "updating all 1,000 rows (5 warmup runs)."}
                            td { span (class="mean") {"43.1"} br {} span (class="deviation") {"0.3"} span (class="factor") {"(1.11)"}}
                            td { span (class="mean") {"49.8"} br {} span (class="deviation") {"0.3"} span (class="factor") {"(1.28)"}}
                            td { span (class="mean") {"48.4"} br {} span (class="deviation") {"0.7"} span (class="factor") {"(1.24)"}}
                            td { span (class="mean") {"73.3"} br {} span (class="deviation") {"0.4"} span (class="factor") {"(1.88)"}}
                        }
                        tr () {
                            th {"partial update" br { } "updating every 10th row for 1,000 rows (3 warmup runs). 16 x CPU slowdown."}
                            td {span (class="mean") {"83.6"} br {} span (class="deviation") {"3.1"} span (class="factor") {"(1.06)"}}
                            td {span (class="mean") {"90.5"} br {} span (class="deviation") {"3.3"} span (class="factor") {"(1.15)"}}
                            td {span (class="mean") {"103.2"} br {} span (class="deviation") {"3.0"} span (class="factor") {"(1.31)"}}
                            td {span (class="mean") {"100.0"} br {} span (class="deviation") {"2.5"} span (class="factor") {"(1.27)"}}
                        }
                        tr () {
                            th {"select row" br { } "highlighting a selected row. (5 warmup runs). 16 x CPU slowdown."}
                            td {span (class="mean") {"13.4"} br {} span (class="deviation") {"0.8"} span (class="factor") {"(1.46)"}}
                            td {span (class="mean") {"17.7"} br {} span (class="deviation") {"1.2"} span (class="factor") {"(1.93)"}}
                            td {span (class="mean") {"24.1"} br {} span (class="deviation") {"1.1"} span (class="factor") {"(2.63)"}}
                            td {span (class="mean") {"21.6"} br {} span (class="deviation") {"0.8"} span (class="factor") {"(2.36)"}}
                        }
                        tr () {
                            th {"swap rows" br { } "swap 2 rows for table with 1,000 rows. (5 warmup runs). 4 x CPU slowdown."}
                            td {span (class="mean") {"26.8"} br {} span (class="deviation") {"0.7"} span (class="factor") {"(1.13)"}}
                            td {span (class="mean") {"26.2"} br {} span (class="deviation") {"0.8"} span (class="factor") {"(1.11)"}}
                            td {span (class="mean") {"160.7"} br {} span (class="deviation") {"1.3"} span (class="factor") {"(6.79)"}}
                            td {span (class="mean") {"27.0"} br {} span (class="deviation") {"0.8"} span (class="factor") {"(1.14)"}}
                        }
                        tr () {
                            th {"remove row" br { } "removing one row. (5 warmup runs). 4 x CPU slowdown."}
                            td {span (class="mean") {"40.9"} br {} span (class="deviation") {"1.0"} span (class="factor") {"(1.10)"}}
                            td {span (class="mean") {"41.3"} br {} span (class="deviation") {"0.9"} span (class="factor") {"(1.11)"}}
                            td {span (class="mean") {"43.5"} br {} span (class="deviation") {"1.3"} span (class="factor") {"(1.17)"}}
                            td {span (class="mean") {"42.1"} br {} span (class="deviation") {"1.1"} span (class="factor") {"(1.13)"}}
                        }
                        tr () {
                            th {"create many rows" br { } "creating 10,000 rows. (5 warmup runs with 1k rows)."}
                            td {span (class="mean") {"433.3"} br {} span (class="deviation") {"1.4"} span (class="factor") {"(1.09)"}}
                            td {span (class="mean") {"569.3"} br {} span (class="deviation") {"2.4"} span (class="factor") {"(1.43)"}}
                            td {span (class="mean") {"619.2"} br {} span (class="deviation") {"3.1"} span (class="factor") {"(1.56)"}}
                            td {span (class="mean") {"2,386.9"} br {} span (class="deviation") {"10.3"} span (class="factor") {"(6.00)"}}
                        }
                        tr () {
                            th {"append rows to large table" br { } "appending 1,000 to a table of 10,000 rows. 2 x CPU slowdown."}
                            td {span (class="mean") {"91.8"}span (class="deviation") {"0.4"} span (class="factor") {"(1.09)"}}
                            td {span (class="mean") {"100.5"}span (class="deviation") {"1.3"} span (class="factor") {"(1.20)"}}
                            td {span (class="mean") {"99.5"}span (class="deviation") {"0.6"} span (class="factor") {"(1.19)"}}
                            td {span (class="mean") {"153.8"}span (class="deviation") {"1.4"} span (class="factor") {"(1.83)"}}
                        }
                        tr () {
                            th {"clear rows" br { } "clearing a table with 1,000 rows. 8 x CPU slowdown. (5 warmup runs)."}
                            td {span (class="mean") {"32.8"}span (class="deviation") {"1.2"} span (class="factor") {"(1.37)"}}
                            td {span (class="mean") {"33.0"}span (class="deviation") {"0.5"} span (class="factor") {"(1.38)"}}
                            td {span (class="mean") {"30.7"}span (class="deviation") {"0.5"} span (class="factor") {"(1.28)"}}
                            td {span (class="mean") {"52.0"}span (class="deviation") {"0.5"} span (class="factor") {"(2.17)"}}
                        }
                        tr () {
                            th {"geometric mean" br { } "of all factors in the table"}
                            th {"1.16"}
                            th {"1.29"}
                            th {"1.67"}
                            th {"1.90"}
                        }
                    }
                }
                
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
