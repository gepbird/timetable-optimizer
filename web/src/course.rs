use boolinator::Boolinator;
use timetable_optimizer_lib::data::Course;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub course: Course,
  pub on_delete: Callback<String>,
  pub on_toggle_visibility: Callback<String>,
}

#[function_component(CourseComponent)]
pub fn course_component(props: &Props) -> Html {
  let course = &props.course;

  let code = course.code.clone();
  let on_delete = {
    let on_delete = props.on_delete.clone();
    move |_| on_delete.emit(code.clone())
  };

  let code = course.code.clone();
  let on_togle_visiblity = {
    let on_toggle_visibility = props.on_toggle_visibility.clone();
    move |_| on_toggle_visibility.emit(code.clone())
  };

  html! {
    <tr class={ classes!(course.is_hidden_by_user.as_some("opacity-50")) }>
      <td>
        <button onclick={on_delete}>{ "Delete" }</button>
        <button onclick={on_togle_visiblity}>{
          if course.is_hidden_by_user { "Show" } else { "Hide" }
        }</button>
      </td>
      <td>{ &course.code }</td>
      <td>{ &course.course_type.to_string() }</td>
      <td>{ &course.location }</td>
      <td>{ &course.occurrence.to_string() }</td>
      <td>{ &course.teacher }</td>
    </tr>
  }
}
