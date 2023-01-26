use crate::{
  utility::css,
  app_color::AppColor
};
use dioxus::{prelude::*, events::MouseData, core::Event};
use lib::NeverEq;
use log::info;

#[derive(Props, NeverEq)]
pub struct ButtonProps<'a> {
  label: String,
  onclick: EventHandler<'a, Event<MouseData>>,
  #[props(optional)]
  background_color: Option<String>,
  #[props(optional)]
  text_color: Option<String>
}

#[allow(non_snake_case)]
pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
  info!("Update - Button");

  // Setup
  let ButtonProps { label, onclick, background_color, text_color } = cx.props;
  let style = styles(background_color.clone(), text_color.clone());

  // Structure
  cx.render(rsx! {
    div {
      class: "{style.button}",
      onclick: |e| { onclick.call(e); },
      div {
        class: "{style.text}",
        "{label}"
      }
    }
  })
}

struct Styles {
  button: String,
  text: String
}

fn styles(
  background_color: Option<String>, 
  text_color: Option<String>
) -> Styles {
  let background_color = background_color
    .unwrap_or_else(|| AppColor::StrongLight.to_string());
  let text_color = text_color
    .unwrap_or_else(|| AppColor::WhiteLight.to_string());

  Styles {
    button: css(format!("
      height: 4em;
      width: 4em;
      border-radius: 0.4em;
      margin-top: 0.4em;
      background-color: {background_color};
      display: flex;
      justify-content: center;
      align-items: center;
    ")),
    text: css(format!("
      color: {text_color};
    "))
  }
}
