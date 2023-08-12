use sycamore::prelude::*;

#[derive(Prop)]
pub struct StepProps<'a, G: Html> {
    children: Children<'a, G>,
    #[builder(default)]
    name: &'a str,
    #[builder(default)]
    x: &'a str,
    #[builder(default)]
    y: &'a str,
    #[builder(default)]
    rotate: i32,
}

#[component]
pub fn Step<'a, G: Html>(cx: Scope<'a>, props: StepProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class_name = format!("step {}", props.name);
    view! { cx,
        div(class = class_name, data-x = props.x, data-y = props.y, data-rotate = props.rotate) {
            (children)
        }
    }
}
