mod app;
mod home;
mod list_item;
mod navbar;
mod page;
mod todo;
mod utility;
mod color;

use app::app;
use sir::global_css;

fn main() {
  global_css!(
    "
    body {
      margin: 0;
    }
  "
  );

  dioxus_logger::init(log::LevelFilter::Info).expect("Failed to init logger");
  dioxus::web::launch(app);
}
