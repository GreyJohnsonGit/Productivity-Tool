use common::todo_item::TodoItem;

pub async fn get_todo_items_all() -> Result<Vec<TodoItem>, String> {
  let client = reqwest::Client::new();
  let response = client.get("http://localhost:3030/todo_items")
    .send()
    .await;
  
  match response {
    Err(_) => Err(String::from("Error")),
    Ok(response) => {
      let json = response.json::<Vec<TodoItem>>().await;
      match json {
        Ok(result) => Ok(result),
        Err(_) => Err(String::from("Error")),
    }
    },
  }
}

pub async fn post_todo_items(todo_item: TodoItem) -> Result<String, String> {
  let client = reqwest::Client::new();
  let response = client.post("http://localhost:3030/todo_items/")
    .json(&todo_item)
    .send()
    .await;
  
  match response {
    Ok(_) => Ok(String::from("Success")),
    Err(_) => Err(String::from("Error")),
  }
}

pub async fn put_todo_items(todo_item: TodoItem) -> Result<String, String> {
  let client = reqwest::Client::new();
  let response = client.put(format!("http://localhost:3030/todo_items/{}", todo_item.id))
    .json(&todo_item)
    .send()
    .await;

  match response {
    Ok(_) => Ok(String::from("Success")),
    Err(_) => Err(String::from("Error")),
  }
}

pub async fn delete_todo_items(id: String) -> Result<String, String> {
  let client = reqwest::Client::new();
  let response = client.delete(format!("http://localhost:3030/todo_items/{id}"))
    .send()
    .await;
  
  match response {
    Ok(_) => Ok(String::from("Success")),
    Err(_) => Err(String::from("Error")),
  }
}