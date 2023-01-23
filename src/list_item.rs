use serde::{Serialize, Deserialize};


#[derive(Clone, Serialize, Deserialize)]
pub struct ListItem {
  pub id: String,
  pub title: String,
  pub description: String,
  pub status: bool,
}

impl PartialEq for ListItem {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl ListItem {
  pub fn new(id: String, title: String, description: String) -> ListItem {
    return ListItem {
      id,
      title,
      description,
      status: false,
    };
  }
}
