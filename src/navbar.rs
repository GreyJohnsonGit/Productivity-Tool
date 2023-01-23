use crate::page::Page;
use dioxus::prelude::*;
use std::rc::Rc;

#[derive(Props)]
pub struct NavBarProps {
  set_page: Rc<dyn Fn(Page)>,
}

impl PartialEq for NavBarProps {
  fn eq(&self, _other: &Self) -> bool {
    false
  }
}

#[allow(non_snake_case)]
pub fn NavBar(cx: Scope<NavBarProps>) -> Element {
  // Setup
  let set_page = cx.props.set_page.as_ref();

  // Structure
  cx.render(rsx! {
    div {
      button {
        onclick: move |_| set_page(Page::Home),
        "Home"
      }
      button {
        onclick: move |_| set_page(Page::Todo),
        "Todo"
      }
    }
  })
}
