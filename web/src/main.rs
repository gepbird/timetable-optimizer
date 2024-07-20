mod app;
mod course;
mod statistics;
mod storage;
mod subject;
mod upload;

use app::AppComponent;

fn main() {
  yew::Renderer::<AppComponent>::new().render();
}
