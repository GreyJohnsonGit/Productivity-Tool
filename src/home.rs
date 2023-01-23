use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
  // State
  let mut count = use_state(&cx, || 0);

  // Structure
  cx.render(rsx! {
    div {
      "Press Me! ({count}) "
      button {
        onclick: move |_| count += 1,
        "Button!"
      }
    }
  })
}
