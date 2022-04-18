mod hook;

pub use self::hook::{use_chessboard, UseChessboard};
use dioxus::{
    core::{Element, Scope},
    core_macro::rsx,
};

fn app(cx: Scope) -> Element {
    let mut count = use_chessboard(cx);

    cx.render(rsx! {
        h1 { "High-Five counter: {count}" }
        // button { onclick: move |_| count += 1, "Up high!" }
        // button { onclick: move |_| count -= 1, "Down low!" }
    })
}
fn main() {
    // launch the web app
    dioxus_web::launch(App);
}
