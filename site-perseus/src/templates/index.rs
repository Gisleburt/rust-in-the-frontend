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
            }

            Step (name = "options-yew-demo", x = 1000, y = 2000) {
                h3 { "Counter Demo" }
                pre {
                    code {
                        "#[function_component]\n"
                        "fn Calculator() -> Html {\n"
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

            Step (name = "options-yew-demo", x = 1000, y = 2000) {
                h3 { "Counter Demo" }
                pre {
                    code {
                        "#[function_component]\n"
                        "fn App() -> Html {\n"
                        "    html! {\n"
                        "        <>\n"
                        "            <h1>{\"Calculator Example\"}</h1>\n"
                        "            <Calculator />\n"
                        "        </>\n"
                        "    }\n"
                        "}\n"
                    }
                }
            }

            Step (name = "options-perseus", x = 0, y = 3000) {
                h3 { "Sycamore / Perseus" }
            }

            Step (name = "options-dioxus", x = 0, y = 4000) {
                h3 { "Dioxus" }
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
