mod app;
mod home;
mod navbar;
mod page;
mod todo;
mod app_color;
mod button;
mod utility;
mod backend;
mod todo_entry;

use app::App;
use sir::global_css;

fn main() {
  global_css!(
    "
    body {
      margin: 0;
      font-family: 'Ubuntu Mono', monospace;
      font-size: 1em;
    }
  "
  );

  dioxus_logger::init(log::LevelFilter::Info).expect("Failed to init logger");
  dioxus_web::launch(App)
}
