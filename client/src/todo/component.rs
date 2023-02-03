use super::lib::Lib;
use crate::{
  todo_entry::TodoEntry,
  utility::css
};
use dioxus::prelude::*;
use common::todo_item::TodoItem;

#[derive(Props, PartialEq)]
pub struct TodoProps<'a> {
  todo_items: &'a UseState<Vec<TodoItem>>,
}

#[allow(non_snake_case)]
pub fn Todo<'a>(cx: Scope<'a, TodoProps<'a>>) -> Element<'a> {
  // State
  let description = use_state(&cx, || String::new());
  let name = use_state(&cx, || String::new());
  let should_add = use_state(&cx, || false);

  let TodoProps { todo_items } = cx.props;
  to_owned![todo_items];
  use_effect(cx, (should_add, todo_items, name, description), 
    |(should_add, todo_items, name, description)| async move {
    if *should_add {
      Lib::add_item(&name, &description, &todo_items).await;
      should_add.modify(|_| false);
    }
  });

  // Setup
  let Styles { todo_list } = styles();
  let a = todo_items.iter();

  // Structure
  cx.render(rsx! {
    div {
      class: "{todo_list}",
      div {
        div {
          // Incomplete Tasks
          todo_items.iter().filter(|i| !i.status).map(|todo_item| rsx!(
            TodoEntry {
              todo_item: todo_item,
              todo_items: todo_items
            }))
          h1 {
            "Complete"
          }
          // Complete Tasks
          todo_items.iter().filter(|i| i.status).map(|todo_item| rsx!(
            TodoEntry {
              todo_item: todo_item,
              todo_items: todo_items
            }))}}
      input {
        placeholder: "Name",
        value: "{name}",
        onchange: move |e| name.modify(|_| e.value.clone())
      }

      input {
        placeholder: "Description",
        value: "{description}",
        onchange: move |e| description.modify(|_| e.value.clone())
      }

      button {
        onclick: |_| should_add.modify(|_| true),
        "Add Item!"
      }}})
}

struct Styles {
  todo_list: String,
}

fn styles() -> Styles {
  Styles {
    todo_list: css(format!("
      width: 100%;
    "))
  }
}