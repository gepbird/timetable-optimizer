mod app;
mod course;
mod statistics;
mod storage;
mod subject;
mod timetable_generator;
mod upload;

use app::AppComponent;

fn main() {
  yew::Renderer::<AppComponent>::new().render();
}
