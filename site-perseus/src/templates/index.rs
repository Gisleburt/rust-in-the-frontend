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

            Step (name = "why", x = 1000) {
                h2 { "Why would you want to do this" }
            }

            Step (name = "why-rust-speed", x = 2000) {
                h3 { "Rust is fast" }
                p { "Rust is a language known for its speed" }
                img (src="/.perseus/static/images/speed-elapsed-seconds.png")
            }

            Step (name = "why-rust-wasm", x = 3000) {
                h3 { "Rust WASM tooling and documentation is very mature" }
                ul {
                    li { "Tier 2: rustup add target wasm32-unknown-unknown" }
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

            Step (name = "why-speed-wasm", x = 4000) {
                h3 { "WASM is fast" }
                p { "WASM is known to provide a secure, high performance runtime for the web" }
                img (src="/.perseus/static/images/speed-wasm.png")
            }

            Step (name = "why-rust-correctness", x = 5000) {
                h3 { "Rust is robust, reliable and 'correct'" }
                pre {
                    code {
                        "async function getUser(email: string): Promise<Result<Error, User>> { \n"
                        "  const response = await fetch(`https://example.com/${email}`);\n"
                        "  const user = await response.json()\n"
                        "  return user.email === email\n"
                        "    ? Result.ok(user)\n"
                        "    : Result.error(new Error('Incorrect user returned');\n"
                        "}\n"
                    }
                }
                pre {
                    code {
                        "async fn getUser(email: &str) -> Result<User, GetUserError> {\n"
                        "    let user: User = get(format!(\"https://example.com/{email}\"))\n"
                        "        .await?\n"
                        "        .json()\n"
                        "        .await?;\n"
                        "    if user.email == email {\n"
                        "        Ok(user)\n"
                        "    } else {\n"
                        "        Err(GetUserError::IncorrectUserReturned)\n"
                        "    }\n"
                        "}\n"
                    }
                }
            }

            Step (name = "options", x = 0, y = 1000) {
                h2 { "What are my options" }
            }

            Step (name = "options-yew", x = 0, y = 2000) {
                h3 { "Yew" }
                ul {
                    li { "Yew looks a lot like React" }
                    li { "Uses a Virtual DOM" }
                    li { "Supports CSR, SSR, and SSG" }
                }
            }

            Step (name = "options-yew-demo-counter", x = 1000, y = 2000) {
                h3 { "Yew Counter" }
                pre {
                    code {
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
            }

            Step (name = "options-yew-demo-counter-usage", x = 2000, y = 2000) {
                h3 { "Yew Counter Usage" }
                pre {
                    code {
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
            }

            Step (name = "options-perseus", x = 0, y = 3000) {
                h3 { "Perseus (Sycamore)" }
                ul {
                    li { "Perseus is to Sycamore what Next is to React" }
                    li { "Does not use Virtual DOM, uses 'fine grain reactivity'" }
                    li { "Supports CSR, SSR, and SSG" }
                }
            }

            Step (name = "options-perseus-demo-counter", x = 1000, y = 3000) {
                h3 { "Perseus Counter" }
                pre {
                    code {
                        "#[component]\n"
                        "#[auto_scope]\n"
                        "fn Counter<G: Html>(cx: Scope, state: &CounterStateRx) -> View<G> {\n"
                        "    view! { cx,\n"
                        "        div {\n"
                        "          p { (state.total.get()) }\n"
                        "          button( on:click = move |_| {state.total.set(*state.total.get() + 1)}) { \"+1\" }\n"
                        "          button( on:click = move |_| {state.total.set(*state.total.get() - 1)}) { \"-1\" }\n"
                        "        }\n"
                        "    }\n"
                        "}\n"
                    }
                }
            }

            Step (name = "options-perseus-demo-state", x = 2000, y = 3000) {
                h3 { "Perseus State" }
                pre {
                    code {
                        "#[derive(Default, Serialize, Deserialize, Clone, ReactiveState)]\n"
                        "#[rx(alias = \"CounterStateRx\")]\n"
                        "struct State {\n"
                        "    total: i32,\n"
                        "}\n"
                    }
                }
            }

            Step (name = "options-perseus-demo-counter-usage", x = 3000, y = 3000) {
                h3 { "Perseus Counter Usage" }
                pre {
                    code {
                        "#[auto_scope]\n"
                        "fn index_page<G: Html>(cx: Scope, state: &CounterStateRx) -> View<G> {\n"
                        "    view! { cx,\n"
                        "        // Don't worry, there are much better ways of styling in Perseus!\n"
                        "        h1 { \"Counter Example - Perseus\" }\n"
                        "        Counter(state)\n"
                        "    }\n"
                        "}\n"
                    }
                }
            }

            Step (name = "options-dioxus", x = 0, y = 4000) {
                h3 { "Dioxus" }
                ul {
                    li { "Similar 'reactivity' approach to Sycamore" }
                    li { "Looks a lot like Sycamore too'" }
                    li { "Supports CSR, SSR, SSG, Desktop, and more!" }
                }
            }

            Step (name = "options-dioxus-demo-counter", x = 1000, y = 4000) {
                h3 { "Dioxus Counter" }
                pre {
                    code {
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
            }

            Step (name = "options-dioxus-demo-counter-usage", x = 2000, y = 4000) {
                h3 { "Dioxus Counter Usage" }
                pre {
                    code {
                        "fn App(cx: Scope) -> Element {\n"
                        "    render!(\n"
                        "        h1 { \"Counter Example - Dioxus\" }\n"
                        "        Counter { }\n"
                        "    )\n"
                        "}\n"
                    }
                }
            }

            Step (name = "options-comparison", x = 0, y = 5000) {
                h3 { "Comparison" }
                ul {
                    li { "Yew is slower than React" }
                    li { "Sycamore is faster" }
                    li { "Dioxus is faster still" }
                    li { "Speed isn't everything" }
                    li { "Dioxus and Sycamore don't allow copy paste html, Yew does" }
                }
            }

            Step (name = "conclusion", x = 0, y = 6000) {
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
        // link (rel = "stylesheet", href = "/.perseus/static/css/impress-demo.css")
        link (rel = "stylesheet", href = "/.perseus/static/css/rust-in-the-frontend.css")
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
