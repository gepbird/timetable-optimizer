mod app;
mod course;

use app::App;

fn main() {
  yew::Renderer::<App>::new().render();
}
