use std::collections::HashMap;
use std::io::Cursor;

use calamine::Xlsx;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use timetable_optimizer_lib::data::{Course, Subject};
use timetable_optimizer_lib::excel_parser;

use crate::statistics::StatisticsComponent;
use crate::storage;
use crate::subject::SubjectComponent;

pub struct App {
  readers: HashMap<String, gloo::file::callbacks::FileReader>,
  subjects: Vec<Subject>,
}

pub enum Msg {
  CoursesUploaded(Vec<gloo::file::File>),
  CourseProcessed(String, Vec<u8>),
  UpdateCourse(String, Box<dyn Fn(&mut Course)>),
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    App {
      readers: HashMap::default(),
      subjects: storage::load_subjects(),
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::CoursesUploaded(files) => {
        self.subjects.clear();
        for file in files {
          let file_name = file.name();
          let link = ctx.link().clone();
          let reader = gloo::file::callbacks::read_as_bytes(&file, move |bytes| {
            link.send_message(Msg::CourseProcessed(file_name, bytes.unwrap()));
          });
          self.readers.insert(file.name(), reader);
        }
        false
      }
      Msg::CourseProcessed(file_name, bytes) => {
        self.readers.remove(&file_name);
        let cursor = Cursor::new(bytes);
        let mut excel: Xlsx<_> = calamine::open_workbook_from_rs(cursor).unwrap();
        let subject = excel_parser::parse_subject(file_name, &mut excel);
        self.subjects.push(subject);
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
    let on_courses_change = ctx.link().callback(move |e: Event| {
      let input: HtmlInputElement = e.target_unchecked_into();
      let files = input.files().unwrap();
      let gloo_files = (0..files.length())
        .map(|i| files.get(i).unwrap())
        .map(gloo::file::File::from)
        .collect();
      Msg::CoursesUploaded(gloo_files)
    });

    html! {
      <main class="min-h-screen bg-gray-800 text-white">
        <label>{ "Subjects:" }</label>
        <input type="file" multiple=true onchange={on_courses_change} />
        { self.view_all_courses(ctx) }
        <StatisticsComponent subjects={self.subjects.clone()} />
      </main>
    }
  }
}
impl App {
  fn view_all_courses(&self, ctx: &Context<Self>) -> Html {
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
      { for self.subjects.iter().map(|s| {
        html! {
          <SubjectComponent subject={s.clone()} on_delete={on_delete.clone()} on_toggle_visibility={on_toggle_visibility.clone()} />
        }
      }) }
    }
  }
}
