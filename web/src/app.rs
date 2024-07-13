use yew::prelude::*;

use timetable_optimizer_lib::data::{Course, Subject};

use crate::statistics::StatisticsComponent;
use crate::storage;
use crate::subject::SubjectsComponent;
use crate::upload::UploadComponent;

pub struct App {
  subjects: Vec<Subject>,
}

pub enum Msg {
  SubjectsUploaded(Vec<Subject>),
  UpdateCourse(String, Box<dyn Fn(&mut Course)>),
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    App {
      subjects: storage::load_subjects(),
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::SubjectsUploaded(subjects) => {
        self.subjects = subjects;
        storage::save_subjects(&self.subjects);
        true
      }
      Msg::UpdateCourse(course_code, update_fn) => {
        let course = self
          .subjects
          .iter_mut()
          .find_map(|subject| {
            subject.courses.iter_mut().find_map(|one_of_courses| {
              one_of_courses
                .iter_mut()
                .find(|course| course.code == course_code)
            })
          })
          .unwrap();
        update_fn(course);
        storage::save_subjects(&self.subjects);
        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let on_courses_change = ctx.link().callback(move |subjects| {
      Msg::SubjectsUploaded(subjects)
    });
    let on_delete = ctx.link().callback(move |course_code: String| {
      Msg::UpdateCourse(course_code, Box::new(|c: &mut Course| c.is_deleted = true))
    });
    let on_toggle_visibility = ctx.link().callback(move |course_code: String| {
      Msg::UpdateCourse(
        course_code,
        Box::new(|c: &mut Course| c.is_hidden_by_user = !c.is_hidden_by_user),
      )
    });

    html! {
      <main class="min-h-screen bg-gray-800 text-white">
        <label>{ "Subjects:" }</label>
        <UploadComponent on_files_processed={on_courses_change}/>
        <SubjectsComponent
          subjects={self.subjects.clone()}
          on_delete={on_delete.clone()}
          on_toggle_visibility={on_toggle_visibility.clone()}
        />
        <StatisticsComponent subjects={self.subjects.clone()} />
      </main>
    }
  }
}
