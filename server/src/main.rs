use std::convert::Infallible;
use common::todo_item::TodoItem;
use mongodb::{Client, options::ClientOptions, bson::doc, Database, Collection};
use warp::Filter;
use futures::TryStreamExt;

#[tokio::main]
async fn main() {
  let db = get_db().await;
  let db2 = get_db().await;
  let db3 = get_db().await;
  
  let api = warp::path!("todo_items")
    .and(warp::get())
    .and(with_db(db))
    .and_then(|db| get_todo_items(db))
    .or(
      warp::path!("todo_items")
      .and(warp::post())
      .and(warp::body::content_length_limit(1024 * 16))
      .and(warp::body::json::<TodoItem>())
      .and(with_db(db2))
      .and_then(|item, db| insert_todo_items(item, db))
    )
    .or(
      warp::path!("todo_items" / String)
      .and(warp::delete())
      .and(with_db(db3))
      .and_then(|id, db| delete_todo_items(id, db))
    );
  
  let cors = warp::cors()
    .allow_any_origin()
    .allow_headers(vec!["User-Agent", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers", "content-type"])
    .allow_methods(vec!["POST", "GET", "DELETE", "PUT"]);

  let routes = api.with(cors);
  warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}


async fn get_db() -> Database {
  let mut client_options = match 
  ClientOptions::parse("mongodb://192.168.0.16").await {
    Ok(options) => options,
    Err(_) => todo!(),
  };
  
  client_options.app_name = Some(String::from("Productivity-Tool-Server"));
  
  let client = match
  Client::with_options(client_options) {
    Ok(client) => client,
    Err(_) => todo!()
  };
  
  client.database("Productivity-Tool")
}

fn with_db(db: Database) -> impl Filter<Extract = (Database,), Error = std::convert::Infallible> + Clone {
  warp::any().map(move || db.clone())
}

async fn get_todo_items(db: Database) -> Result<impl warp::Reply, Infallible> {
  let documents = get_todo_items_db(db).await;
  Ok(warp::reply::json(&documents))
}

async fn insert_todo_items(item: TodoItem, db: Database) -> Result<impl warp::Reply, Infallible> {
  insert_todo_items_db(item, db).await;
  Ok(warp::reply())
}

async fn delete_todo_items(id: String, db: Database) -> Result<impl warp::Reply, Infallible> {
  delete_todo_items_db(id, db).await;
  Ok(warp::reply())
}

async fn get_todo_items_db(db: Database) -> Vec<TodoItem> {
  let collection = AppCollections::todo_items(db);
  let cursor = match collection.find(doc! {}, None).await {
    Ok(cursor) => cursor,
    Err(_) => todo!(),
  };
  
  match cursor.try_collect().await {
    Ok(documents) => documents,
    Err(_) => vec![],
  }
}

async fn insert_todo_items_db(document: TodoItem, db: Database) {
  let collection = AppCollections::todo_items(db);
  match collection.insert_one(document, None).await {
    Ok(_) => {},
    Err(_) => todo!(),
  };
}

async fn delete_todo_items_db(id: String, db: Database) {
  let collection = AppCollections::todo_items(db);
  match collection.delete_one(doc! { "id": id }, None).await {
    Ok(_) => {},
    Err(_) => todo!(),
  };
}

pub struct AppCollections {}

impl AppCollections {
  pub fn todo_items(db: Database) -> Collection<TodoItem> {
    db.collection::<TodoItem>("TodoItems")
  }
}