#![allow(dead_code)]
use stylist::Style;

pub fn css(css: String) -> String {
  String::from(Style::new(css).expect("Failed to create style").get_class_name())
}

