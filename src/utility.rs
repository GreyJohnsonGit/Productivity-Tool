use dioxus::prelude::*;
use std::rc::Rc;
use stylist::Style;

pub fn decompose<'a, T>(state: &'a UseState<T>) -> (&T, Rc<dyn Fn(T)>, &'a UseState<T>) {
  return (state.get(), state.setter(), state);
}

pub fn css(css: String) -> String {
  String::from(Style::new(css).expect("Failed to create style").get_class_name())
}