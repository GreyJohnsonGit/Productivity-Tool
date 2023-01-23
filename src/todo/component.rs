use super::lib::Factory;
use crate::color::Color;
use crate::list_item::ListItem;
use crate::utility::{self, css};
use dioxus::prelude::*;

#[derive(Props)]
pub struct TodoProps<'a> {
  list_state: &'a UseState<Vec<ListItem>>,
}

impl PartialEq for TodoProps<'_> {
  fn eq(&self, _other: &Self) -> bool {
    false
  }
}

#[allow(non_snake_case)]
pub fn Todo<'a>(cx: Scope<'a, TodoProps<'a>>) -> Element {
  // State
  let (
    description, 
    set_description, 
    _
  ) = utility::decompose(use_state(&cx, || String::new())); 
  let (
    name, 
    set_name, 
    _
  ) = utility::decompose(use_state(&cx, || String::new()));

  // Setup
  let (list, _set_list, list_state) = utility::decompose(cx.props.list_state);
  let add_item = Factory::add_item(list_state);
  let remove_item = |id| Factory::remove_item(id, list_state);
  let styles = styles(); 
  let styles_list_item = styles.list_item.as_str();

  // Structure
  cx.render(rsx! {
    div {
      div {
        list.iter().map(|list_item| rsx!(
          div {
            key: "{list_item.id}",
            class: "{styles_list_item}",
            p { "{list_item.id}" }
            p { "{list_item.title}" }
            p { "{list_item.description}" }
            div {
              onclick: remove_item(list_item.id.clone()),
              "X"
            }}))}
      input {
        placeholder: "Name",
        value: "{name}",
        onchange: move |e| set_name(e.value.clone())
      }

      input {
        placeholder: "Description",
        value: "{description}",
        onchange: move |e| set_description(e.value.clone())
      }

      button {
        onclick: add_item(name, description),
        "Add Item!"
      }}})
}

struct Styles {
  list_item: String,
}

fn styles() -> Styles {
  let strong_light = Color::StrongLight.as_str();
  let weak = Color::Weak.as_str();
  let white_light = Color::WhiteLight.as_str();
  let _ = Color::WhiteDark;

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