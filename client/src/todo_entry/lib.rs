use common::todo_item::{TodoItem, self};
use dioxus::prelude::{UseState, to_owned};
use crate::{backend::{
  delete_todo_items, 
  get_todo_items_all,
  put_todo_items
}, todo};

pub struct Lib;

impl Lib {
  pub async fn remove_item<'a>(
    id: &str,
    todo_items: &'a UseState<Vec<TodoItem>>,
  ) { 
    match delete_todo_items(String::from(id)).await {
      Ok(_) => {},
      Err(_) => return,
    }
    
    match get_todo_items_all().await {
      Ok(remote_todo_items) => todo_items.modify(|_| remote_todo_items),
      Err(_) => return,
    };
  }

  pub async fn toggle_item<'a>(
    todo_item: &'a TodoItem,
    todo_items: &'a UseState<Vec<TodoItem>>,
  ) {
    let mut updated_todo_item = todo_item.clone();
    updated_todo_item.status = !todo_item.status;

    match put_todo_items(updated_todo_item).await {
      Ok(_) => {},
      Err(_) => return,
    }

    match get_todo_items_all().await {
      Ok(remote_todo_items) => todo_items.modify(|_| remote_todo_items),
      Err(_) => return,
    }
  }
}
