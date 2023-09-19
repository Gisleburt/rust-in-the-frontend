use sycamore::prelude::*;

#[derive(Prop)]
pub struct CodeProps<'a, G: Html> {
    children: Children<'a, G>,
    #[builder(default)]
    language: &'a str,
}

#[component]
pub fn Code<'a, G: Html>(cx: Scope<'a>, props: CodeProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("language-{}", props.language);
    view! { cx,
        pre {
            code(class = class) {
                (children)
            }
        }
    }
}
