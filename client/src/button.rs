use crate::{
  utility::css,
  app_color::AppColor
};
use dioxus::{prelude::*, events::MouseData, core::Event};
use lib::NeverEq;

#[derive(Props, NeverEq)]
pub struct ButtonProps<'a> {
  label: &'a str,
  onclick: EventHandler<'a, Event<MouseData>>,
  #[props(optional)]
  background_color: Option<&'a str>,
  #[props(optional)]
  text_color: Option<&'a str>
}

#[allow(non_snake_case)]
pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
  // State
  let is_hovered = use_state(cx, || false);

  // Setup
  let ButtonProps { label, onclick, background_color, text_color } = cx.props;
  let style = styles(
    background_color.clone(), 
    text_color.clone(), 
    *is_hovered.get()
  );

  // Structure
  cx.render(rsx! {
    div {
      class: "{style.button}",
      onclick: |e| { onclick.call(e); },
      onmouseenter: |_| { is_hovered.modify(|_| true)},
      onmouseleave: |_| { is_hovered.modify(|_| false)},
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
  background_color: Option<&str>, 
  text_color: Option<&str>,
  is_hovered: bool
) -> Styles {
  let background_color = background_color
    .unwrap_or_else(|| AppColor::StrongLight.as_str()).to_string();
  let text_color = text_color
    .unwrap_or_else(|| AppColor::PaleSecondary.as_str()).to_string();

  let darken_css = match is_hovered {
    true => "filter: brightness(85%);",
    false => "",
  };

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
      {darken_css}
    ")),
    text: css(format!("
      color: {text_color};
    "))
  }
}
