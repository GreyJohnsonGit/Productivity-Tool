use crate::utility::css;
use crate::home::Home;
use crate::list_item::ListItem;
use crate::navbar::NavBar;
use crate::page::Page;
use crate::todo::Todo;
use crate::color::Color;
use dioxus::prelude::*;
use sir::AppStyle;

pub fn app(cx: Scope) -> Element {
  // State
  let page_state = use_state(&cx, || Page::Home);
  let list_state = use_state(&cx, || Vec::<ListItem>::new());

  use_effect(&cx, (list_state,), |(list_state,)| async move {
    list_state.setter()
  });

  // Setup
  let page = &*page_state.current();
  let styles = styles();

  // Structure
  cx.render(rsx! {
    AppStyle {}
    div {
      class: "{styles.body}",
      NavBar {
        set_page: page_state.setter()
      }
      match page {
        Page::Home => Home(cx),
        Page::Todo => rsx!(cx, Todo { list_state: list_state })
      }
    }
  })
}

struct Styles {
  body: String,
}

fn styles() -> Styles {
  let white = Color::StrongDark.as_str();

  Styles {
    body: css(format!("
      background-color: {white};
      height: 100vh;
      padding: 2em;
      display: flex;
      flex-direction: column;
    ")),
  }
}
