use crate::course::CourseComponent;
use timetable_optimizer_lib::data::{Course, Subject};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SubjectProps {
  pub subject: Subject,
  pub on_delete: Callback<String>,
  pub on_toggle_visibility: Callback<String>,
}

#[function_component(SubjectComponent)]
pub fn subject_component(props: &SubjectProps) -> Html {
  let courses: Vec<&Course> = props.subject.courses.iter().flatten().collect();
  html! {
    <div class="my-6">
      <h2>{ &props.subject.name }</h2>
      if !courses.is_empty() {
        <table>
          <thead>
            <tr>
              <th></th>
              <th>{ "Code" }</th>
              <th>{ "Type" }</th>
              <th>{ "Location" }</th>
              <th>{ "Occurrence" }</th>
              <th>{ "Teacher" }</th>
            </tr>
          </thead>
          <tbody>
            { for courses.into_iter().filter(|course| !course.is_deleted).map(|course| {
              html! {
                <CourseComponent course={course.clone()} on_delete={props.on_delete.clone()} on_toggle_visibility={props.on_toggle_visibility.clone()} />
              }
            }) }
          </tbody>
        </table>
      }
    </div>
  }
}
