use crate::{
  utility::css,
  home::Home,
  navbar::NavBar,
  page::Page,
  todo::Todo,
  app_color::AppColor,
  backend::get_todo_items_all
};
use common::todo_item::{TodoItem, self};
use dioxus::prelude::*;
use log::info;
use sir::AppStyle;

#[allow(non_snake_case)]
pub fn App(cx: Scope) -> Element {
  info!("Update - App");
  
  // State
  let page = use_state(&cx, || Page::Home);
  let todo_items = use_state(&cx, || Vec::<TodoItem>::new());

  cx.use_hook(|| {
    cx.spawn({
      to_owned![todo_items];
      async move {
        match get_todo_items_all().await {
          Ok(items) => todo_items.modify(|_| items),
          Err(_) => todo_items.modify(|_| vec![]),
        };
      }
    });
  });
  

  // Setup
  let styles = styles();

  // Structure
  cx.render(rsx! {
    AppStyle {}
    div {
      class: "{styles.body}",
      
      div {
        class: "{styles.function_bar}",
        NavBar {
          page: page
        }}

      div {
        class: "{styles.content_panel}",
        match page.get() {
          Page::Home => Home(cx),
          Page::Todo => cx.render(rsx!(Todo { todo_items: todo_items }))
        }}
    }
  })
}

struct Styles {
  body: String,
  function_bar: String,
  content_panel: String
}

fn styles() -> Styles {
  let strong_dark = AppColor::StrongDark.as_str();
  let dark_white = AppColor::WhiteDark.as_str();

  Styles {
    body: css(format!("
      background-color: {strong_dark};
      height: 100vh;
      display: flex;
    ")),
    function_bar: css(format!("
      width: 5em;
      height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: stretch;
      background-color: {dark_white};
    ")),
    content_panel: css(format!("
      padding: 2em;
      display: flex;
    "))
  }
}
