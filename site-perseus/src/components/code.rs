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
    let pre_class = format!("theme-ally-dark");
    let code_class = format!("language-{}", props.language);
    view! { cx,
        pre(class = pre_class) {
            code(class = code_class) {
                (children)
            }
        }
    }
}
