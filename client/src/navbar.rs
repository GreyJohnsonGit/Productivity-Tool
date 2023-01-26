use crate::{page::Page, utility::css, button::Button};
use dioxus::prelude::*;
use lib::NeverEq;
use log::info;

#[derive(Props, NeverEq)]
pub struct NavBarProps<'a> {
  page: &'a UseState<Page>,
}

#[allow(non_snake_case)]
pub fn NavBar<'a>(cx: Scope<'a, NavBarProps>) -> Element<'a> {
  info!("Update - NavBar");
  
  // Setup
  let styles = styles();
  let NavBarProps { page } = cx.props;

  // Structure
  cx.render(rsx! {
    div {
      class: "{styles.container}",
      Button {
        onclick: move |_| page.modify(|_| Page::Home),
        label: "Home".to_string()
      },
      Button {
        onclick: move |_| page.modify(|_| Page::Todo),
        label: "Todo".to_string()
      },
    }
  })
}

struct Styles {
  container: String,
}

fn styles() -> Styles {
  Styles {
    container: css(format!("
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: flex-start;
    "))
  }
}