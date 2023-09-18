use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;


#[derive(Default, Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "CounterStateRx")]
struct State {
    total: i32,
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> State {
    State::default()
}

#[component]
#[auto_scope]
fn Counter<G: Html>(cx: Scope, state: &CounterStateRx) -> View<G> {
    view! { cx,
        div {
          p { (state.total.get()) }
          button( on:click = move |_| {state.total.set(*state.total.get() + 1)}) { "+1" }
          button( on:click = move |_| {state.total.set(*state.total.get() - 1)}) { "-1" }
        }
    }
}

#[auto_scope]
fn index_page<G: Html>(cx: Scope, state: &CounterStateRx) -> View<G> {
    view! { cx,
        // Don't worry, there are much better ways of styling in Perseus!
        h1 { "Counter Example - Perseus" }
        Counter(state)
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Demo - Perseus" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .head(head)
        .build()
}
