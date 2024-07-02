mod app;
mod course;
mod statistics;
mod subject;

use app::App;

fn main() {
  yew::Renderer::<App>::new().render();
}
