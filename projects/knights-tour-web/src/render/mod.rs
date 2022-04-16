use dioxus::prelude::*;

// First, declare a root component
fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "hello world" }
    })
}

fn main() {
    // Next, create a new VirtualDom using this app as the root component.
    let mut dom = VirtualDom::new(app);

    // The initial render of the dom will generate a stream of edits for the real dom to apply
    let mutations = dom.rebuild();

    // Somehow, you can apply these edits to the real dom
    apply_edits_to_real_dom(mutations);
}
