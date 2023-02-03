use super::lib::Lib;
use crate::app_color::AppColor;
use crate::button::Button;
use crate::utility::css;
use dioxus::prelude::*;
use common::todo_item::TodoItem;

#[derive(Props, PartialEq)]
pub struct TodoEntryProps<'a>  {
  todo_items: &'a UseState<Vec<TodoItem>>,
  todo_item: &'a TodoItem,
}

#[allow(non_snake_case)]
pub fn TodoEntry<'a>(cx: Scope<'a, TodoEntryProps<'a>>) -> Element<'a> {
  // State
  let TodoEntryProps { todo_items, todo_item } = cx.props;
  to_owned![todo_items, todo_item];
  let should_delete = use_state(&cx, || false);
  let should_toggle = use_state(&cx, || false);

  use_effect(cx,
    (todo_item, should_delete, should_toggle, todo_items), 
    |(todo_item, should_delete, should_toggle, todo_items)| async move {
      if *should_delete {
        Lib::remove_item(todo_item.id.as_str(), &todo_items).await;
        should_delete.modify(|_| false);
      }

      if *should_toggle {
        Lib::toggle_item(&todo_item, &todo_items).await;
        should_toggle.modify(|_| false);
      }
  });

  // Setup
  let Styles { list_item, info_segment, action_segment, todo_title } = styles(); 

  // Structure
  cx.render(rsx! {
    div {
      key: "{todo_item.id}",
      class: "{list_item}",
      div {
        class: "{info_segment}",
        p {
          class: "{todo_title}",
          "{todo_item.title}" 
        }
        p { "{todo_item.description}" }
      }
      div {
        class: "{action_segment}",
        Button {
          label: "Delete",
          onclick: move |_| { should_delete.modify(|_| true); },
          background_color: AppColor::Error.as_str()
        }
        Button {
          label: "Modify",
          onclick: move |_| { },
          background_color: AppColor::Warning.as_str()
        }
        Button {
          label: "Complete",
          onclick: move |_| { should_toggle.modify(|_| true); },
          background_color: AppColor::SuccessDark.as_str()
        }
      }
    }
  })
}

struct Styles {
  list_item: String,
  info_segment: String,
  action_segment: String,
  todo_title: String
}

fn styles() -> Styles {
  let strong_light = AppColor::StrongLight.as_str();
  let weak = AppColor::Weak.as_str();
  let pale_light = AppColor::PaleSecondary.as_str();
  let pale_dark = AppColor::PalePrimary.as_str();

  Styles {
    list_item: css(format!("
      display: flex;
      flex-direction: row;
      justify-content: space-between;
      border: 0.1em solid {weak};
      border-radius: 0.2em;
      background-color: {strong_light};
      color: {pale_light};
      padding: 0.1em;
      margin: 0.1em;
    ")),
    info_segment: css(format!("
      display: flex;
      flex-direction: column;
      background-color: {strong_light};
      color: {pale_dark};
      padding: 0.1em;
      margin: 0.1em;
    ")),
    action_segment: css(format!("
      display: flex;
      flex-direction: column;
      background-color: {strong_light};
      padding: 0.1em;
      margin: 0.1em;
    ")),
    todo_title: css(format!("
      color: {pale_light};
      font-size: 1.5em;
    ")),
  }
}