use timetable_optimizer_lib::{data::Subject, stats};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub subjects: Vec<Subject>,
}

#[function_component(StatisticsComponent)]
pub fn statistics_component(props: &Props) -> Html {
  html! {
    <>
      <h1>{ "Statistics" }</h1>
      <p>{ format!("Total courses inputted: {}", stats::count_all_courses(&props.subjects)) }</p>
      <p>{ format!("Total courses in a timetable: {}", stats::count_course_per_timetable(&props.subjects)) }</p>
      <p>{ format!("Total possible timetables: {}", stats::count_all_timetables(&props.subjects)) }</p>
    </>
  }
}
