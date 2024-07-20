use yew::prelude::*;

use crate::statistics::StatisticsComponent;
use crate::storage;
use crate::subject::SubjectsComponent;
use crate::upload::UploadComponent;

#[function_component(AppComponent)]
pub fn app_component() -> Html {
  let subjects = use_state(storage::load_subjects);

  let on_subjects_processed = {
    let subjects = subjects.clone();
    move |new_subjects| {
      storage::save_subjects(&new_subjects);
      subjects.set(new_subjects);
    }
  };
  let on_delete = {
    let subjects = subjects.clone();
    move |course_code: String| {
      let mut new_subjects = (*subjects).clone();
      let course = new_subjects
        .iter_mut()
        .find_map(|subject| {
          subject.courses.iter_mut().find_map(|one_of_courses| {
            one_of_courses
              .iter_mut()
              .find(|course| course.code == course_code)
          })
        })
        .unwrap();
      course.is_deleted = true;
      storage::save_subjects(&new_subjects);
      subjects.set(new_subjects);
    }
  };
  let on_toggle_visibility = {
    let subjects = subjects.clone();
    move |course_code: String| {
      let mut new_subjects = (*subjects).clone();
      let course = new_subjects
        .iter_mut()
        .find_map(|subject| {
          subject.courses.iter_mut().find_map(|one_of_courses| {
            one_of_courses
              .iter_mut()
              .find(|course| course.code == course_code)
          })
        })
        .unwrap();
      course.is_hidden_by_user = !course.is_hidden_by_user;
      storage::save_subjects(&new_subjects);
      subjects.set(new_subjects);
    }
  };

  html! {
    <main class="min-h-screen bg-gray-800 text-white">
      <label>{ "Subjects:" }</label>
      <UploadComponent on_files_processed={on_subjects_processed}/>
      <SubjectsComponent
        subjects={(*subjects).clone()}
        on_delete={on_delete.clone()}
        on_toggle_visibility={on_toggle_visibility.clone()}
      />
      <StatisticsComponent subjects={(*subjects).clone()} />
    </main>
  }
}
