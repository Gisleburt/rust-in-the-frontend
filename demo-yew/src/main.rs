use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <h1>{"Counter Example - Yew"}</h1>
            <Counter />
        </>
    }
}

#[function_component]
fn Counter() -> Html {
    let counter = use_state(|| 0);
    let increment = {
        let counter = counter.clone();
        move |_| { counter.set(*counter + 1); }
    };
    let decrement = {
        let counter = counter.clone();
        move |_| { counter.set(*counter - 1); }
    };

    html! {
        <div>
            <p>{ *counter }</p>
            <button onclick={increment}>{ "+1" }</button>
            <button onclick={decrement}>{ "-1" }</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
