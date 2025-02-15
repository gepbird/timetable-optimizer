mod course;
mod statistics;
mod storage;
mod subject;
mod timetable_generator;
mod upload;

use timetable_optimizer_lib::data::UpdateSubjectsByCourseCode;
use yew::prelude::*;

use crate::statistics::StatisticsComponent;
use crate::subject::SubjectsComponent;
use crate::upload::UploadComponent;
use crate::timetable_generator::TimetableGenerator;

#[function_component(AppComponent)]
pub fn app_component() -> Html {
  let subjects = use_state(storage::load_subjects);

  let update_subjects = {
    let subjects = subjects.clone();
    move |new_subjects| {
      storage::save_subjects(&new_subjects);
      subjects.set(new_subjects);
    }
  };

  let on_delete = {
    let subjects = subjects.clone();
    let update_subjects = update_subjects.clone();
    move |course_code: String| {
      let mut new_subjects = (*subjects).clone();
      new_subjects.update_subjects_by_course_code(
        course_code.clone(),
        Box::new(|course| course.is_deleted = true),
      );
      update_subjects(new_subjects);
    }
  };
  let on_toggle_visibility = {
    let subjects = subjects.clone();
    let update_subjects = update_subjects.clone();
    move |course_code: String| {
      let mut new_subjects = (*subjects).clone();
      new_subjects.update_subjects_by_course_code(
        course_code.clone(),
        Box::new(|course| course.is_hidden_by_user = !course.is_hidden_by_user),
      );
      update_subjects(new_subjects);
    }
  };

  html! {
    <main class="min-h-screen bg-gray-800 text-white">
      <label>{ "Subjects:" }</label>
      <UploadComponent on_files_processed={update_subjects}/>
      <SubjectsComponent
        subjects={(*subjects).clone()}
        on_delete={on_delete.clone()}
        on_toggle_visibility={on_toggle_visibility.clone()}
      />
      <StatisticsComponent subjects={(*subjects).clone()} />
      <TimetableGenerator subjects={(*subjects).clone()} />
    </main>
  }
}
