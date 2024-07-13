mod app;
mod course;
mod statistics;
mod storage;
mod subject;
mod upload;

use app::App;

fn main() {
  yew::Renderer::<App>::new().render();
}
