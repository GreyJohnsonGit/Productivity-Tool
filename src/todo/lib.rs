use crate::list_item::ListItem;
use dioxus::{core::UiEvent, events::MouseData, prelude::UseState};
use log::info;
use uuid::Uuid;

pub struct Factory;

impl Factory {
  pub fn add_item<'a>(
    list_state: &'a UseState<Vec<ListItem>>
  ) -> impl Fn(&'a String, &'a String) -> Box<dyn Fn(UiEvent<MouseData>) + 'a> + '_{
    move |name, description| {
      Box::new(move |_| {
        let (list, set_list) = (list_state.get(), list_state.setter());
        let mut new_list = list.to_vec();
        new_list.push(ListItem::new(
          Uuid::new_v4().to_string(),
          name.clone(),
          description.clone(),
        ));
        info!("{} - {}", name, description);
        set_list(new_list);
      })
      
    }
  }

  pub fn remove_item(
    id: String,
    list_state: &UseState<Vec<ListItem>>
  ) -> impl Fn(UiEvent<MouseData>) -> () + '_ {
    let (list, set_list) = (list_state.get(), list_state.setter());
    return move |_| {
      let new_list = list
        .into_iter()
        .filter(|item| { item.id != id })
        .map(|item| { item.clone() })
        .collect::<Vec<_>>();

      set_list(new_list);
    }
  }
}
