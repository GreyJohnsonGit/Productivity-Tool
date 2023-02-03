use common::todo_item::TodoItem;
use dioxus::prelude::UseState;
use uuid::Uuid;
use crate::backend::{
  get_todo_items_all,
  post_todo_items
};

pub struct Lib;

impl Lib {
  pub async fn add_item<'a>(
    name: &'a UseState<String>,
    description: &'a UseState<String>,
    todo_items: &'a UseState<Vec<TodoItem>>
  ) {
    let new_item = TodoItem::new(
      Uuid::new_v4().to_string(),
      name.to_string(),
      description.to_string(),
    );
    
    match post_todo_items(new_item).await {
        Ok(_) => {},
        Err(_) => return,
    }

    match get_todo_items_all().await {
      Ok(remote_todo_items) => todo_items.modify(|_| remote_todo_items),
      Err(_) => return,
    };
  }
}
