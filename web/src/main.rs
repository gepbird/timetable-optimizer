mod app;
mod course;
mod subject;

use app::App;

fn main() {
  yew::Renderer::<App>::new().render();
}
