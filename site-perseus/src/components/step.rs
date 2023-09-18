use sycamore::prelude::*;

#[derive(Prop)]
pub struct StepProps<'a, G: Html> {
    children: Children<'a, G>,
    #[builder(default)]
    name: &'a str,
    #[builder(default)]
    x: i32,
    #[builder(default)]
    y: i32,
    #[builder(default)]
    rotate: i32,
}

#[component]
pub fn Step<'a, G: Html>(cx: Scope<'a>, props: StepProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    view! { cx,
        div(class = "step", id = props.name, data-x = props.x, data-y = props.y, data-rotate = props.rotate) {
            (children)
        }
    }
}
