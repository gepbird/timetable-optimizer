mod app;
mod course;
mod statistics;
mod storage;
mod subject;

use app::App;

fn main() {
  yew::Renderer::<App>::new().render();
}
