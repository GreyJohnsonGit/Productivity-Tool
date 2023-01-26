use super::lib::Lib;
use crate::app_color::AppColor;
use crate::utility::css;
use dioxus::prelude::*;
use common::todo_item::TodoItem;
use log::info;

#[derive(Props, PartialEq)]
pub struct TodoProps<'a> {
  todo_items: &'a UseState<Vec<TodoItem>>,
}

#[allow(non_snake_case)]
pub fn Todo<'a>(cx: Scope<'a, TodoProps<'a>>) -> Element<'a> {
  info!("Update - Todo");

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


  // Structure
  cx.render(rsx! {
    div {
      div {
        div {
          todo_items.iter().map(|todo_item| rsx!(
            TodoList {
              todo_item: todo_item.clone(),
              todo_items: todo_items
            }
          ))}
        }
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

#[derive(Props, PartialEq)]
pub struct TodoListProps<'a>  {
  todo_items: &'a UseState<Vec<TodoItem>>,
  todo_item: TodoItem
}

#[allow(non_snake_case)]
pub fn TodoList<'a>(cx: Scope<'a, TodoListProps<'a>>) -> Element<'a> {
  // State
  let TodoListProps { todo_items, todo_item } = cx.props;
  to_owned![todo_items, todo_item];
  let should_delete = use_state(&cx, || false);

  let todo_items = todo_items.clone();
  use_effect(cx,
    (&todo_item, should_delete, &todo_items), 
    |(todo_item, should_delete, todo_items)| async move {
      if *should_delete {
        Lib::remove_item(todo_item.id.as_str(), &todo_items).await;
        should_delete.modify(|_| false);
      }
  });

  // Setup
  let Styles { list_item } = styles(); 
  let todo_item_style = list_item.as_str();

  // Structure
  cx.render(rsx! {
    div {
      key: "{todo_item.id}",
      class: "{todo_item_style}",
      p { "{todo_item.id}" }
      p { "{todo_item.title}" }
      p { "{todo_item.description}" }
      div {
        onclick: move |_| { should_delete.modify(|_| true); },
        "X"
      }}
  })
}

struct Styles {
  list_item: String,
}

fn styles() -> Styles {
  let strong_light = AppColor::StrongLight.as_str();
  let weak = AppColor::Weak.as_str();
  let white_light = AppColor::WhiteLight.as_str();
  let _ = AppColor::WhiteDark;

  Styles {
    list_item: css(format!("
      width: 20em;
      border: 0.1em solid {weak};
      border-radius: 0.2em;
      background-color: {strong_light};
      color: {white_light};
      padding: 0.1em;
      margin: 0.1em;
    ")),
  }
}