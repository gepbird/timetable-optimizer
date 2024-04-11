use std::collections::HashMap;
use std::io::Cursor;

use calamine::Xlsx;
use gloo::storage::{LocalStorage, Storage};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use timetable_optimizer_lib::data::{Course, Subject};
use timetable_optimizer_lib::{excel_parser, stats};

pub struct App {
  readers: HashMap<String, gloo::file::callbacks::FileReader>,
  subjects: Option<Vec<Subject>>,
}

pub enum Msg {
  SubjectUploaded(gloo::file::File),
  SubjectProcessed(String, Vec<u8>),
  CourseUploaded(String, gloo::file::File),
  CourseProcessed(String, String, Vec<u8>),
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    App {
      readers: HashMap::default(),
      subjects: Self::load_subjects(),
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::SubjectUploaded(file) => {
        let file_name = file.name();
        let link = ctx.link().clone();
        let reader = gloo::file::callbacks::read_as_bytes(&file, move |bytes| {
          link.send_message(Msg::SubjectProcessed(file_name, bytes.unwrap()));
        });
        self.readers.insert(file.name(), reader);
        false
      }
      Msg::SubjectProcessed(file_name, bytes) => {
        self.readers.remove(&file_name);
        let cursor = Cursor::new(bytes);
        let mut excel: Xlsx<_> = calamine::open_workbook_from_rs(cursor).unwrap();
        self.subjects = Some(excel_parser::parse_subjects(&mut excel));
        self.save_subjects();
        true
      }
      Msg::CourseUploaded(subject_name, file) => {
        let file_name = file.name();
        let link = ctx.link().clone();
        let reader = gloo::file::callbacks::read_as_bytes(&file, move |bytes| {
          link.send_message(Msg::CourseProcessed(
            file_name,
            subject_name,
            bytes.unwrap(),
          ));
        });
        self.readers.insert(file.name(), reader);
        false
      }
      Msg::CourseProcessed(file_name, subject_name, bytes) => {
        self.readers.remove(&file_name);
        let cursor = Cursor::new(bytes);
        let mut excel: Xlsx<_> = calamine::open_workbook_from_rs(cursor).unwrap();
        let subject = self
          .subjects
          .as_mut()
          .unwrap()
          .iter_mut()
          .find(|s| s.name == subject_name)
          .unwrap();
        subject.courses = excel_parser::parse_courses(subject_name.as_str(), &mut excel);
        self.save_subjects();
        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let on_subject_change = ctx.link().callback(move |e: Event| {
      let input: HtmlInputElement = e.target_unchecked_into();
      let files = input.files().unwrap();
      let websys_file = files.get(0).unwrap();
      let gloo_file = gloo::file::File::from(websys_file);
      Msg::SubjectUploaded(gloo_file)
    });

    html! {
      <main class="min-h-screen bg-gray-800 text-white">
        <label>{ "Subject:" }</label>
        <input type="file" onchange={on_subject_change} />
        { self.view_subjects() }
        { self.view_all_courses(ctx) }
        { self.view_stats() }
      </main>
    }
  }
}
impl App {
  fn view_subjects(&self) -> Html {
    html! {
      if let Some(subjects) = &self.subjects {
        <table>
          <thead>
            <tr>
              <th>{ "Name" }</th>
            </tr>
          </thead>
          <tbody>
            { for subjects.iter().map(|s| html! {
              <tr>
                <td>{ &s.name }</td>
              </tr>
            }) }
          </tbody>
        </table>
      }
    }
  }

  fn view_all_courses(&self, ctx: &Context<Self>) -> Html {
    let on_course_change = ctx.link().callback(move |e: Event| {
      let input: HtmlInputElement = e.target_unchecked_into();
      let subject_code = input.get_attribute("data-subject-name").unwrap();
      let files = input.files().unwrap();
      let websys_file = files.get(0).unwrap();
      let gloo_file = gloo::file::File::from(websys_file);
      Msg::CourseUploaded(subject_code, gloo_file)
    });

    html! {
      if let Some(subjects) = &self.subjects {
        { for subjects.iter().map(|s| {
          html! {
            <div class="my-6">
              <label>{ format!("Courses for {}:", &s.name) }</label>
              <input type="file" data-subject-name={s.name.clone()} onchange={&on_course_change} />
              { self.view_courses(s.name.clone()) }
            </div>
          }
        }) }
      }
    }
  }

  fn view_courses(&self, subject_name: String) -> Html {
    let subject = self
      .subjects
      .as_ref()
      .unwrap()
      .iter()
      .find(|s| s.name == subject_name)
      .unwrap();
    let courses: Vec<&Course> = subject.courses.iter().flatten().collect();
    html! {
      if !courses.is_empty() {
        <table>
          <thead>
            <tr>
              <th>{ "Code" }</th>
              <th>{ "Type" }</th>
              <th>{ "Location" }</th>
              <th>{ "Occurrence" }</th>
              <th>{ "Teacher" }</th>
            </tr>
          </thead>
          <tbody>
            { for courses.into_iter().map(|c| html! {
              <tr>
                <td>{ &c.code }</td>
                <td>{ &c.course_type.to_string() }</td>
                <td>{ &c.location }</td>
                <td>{ &c.occurrence.to_string() }</td>
                <td>{ &c.teacher }</td>
              </tr>
            }) }
          </tbody>
        </table>
      }
    }
  }

  fn view_stats(&self) -> Html {
    html! {
      if let Some(subjects) = &self.subjects {
        <h1>{ "Statistics" }</h1>
        <p>{ format!("Total courses inputted: {}", stats::count_all_courses(subjects)) }</p>
        <p>{ format!("Total courses in a timetable: {}", stats::count_course_per_timetable(subjects)) }</p>
        <p>{ format!("Total possible timetables: {}", stats::count_all_timetables(subjects)) }</p>
      }
    }
  }

  fn save_subjects(&self) {
    let subjects = serde_json::to_string(&self.subjects).unwrap();
    LocalStorage::set("subjects", subjects).unwrap();
  }

  fn load_subjects() -> Option<Vec<Subject>> {
    match LocalStorage::get::<String>("subjects") {
      Ok(subjects) => Some(serde_json::from_str::<Vec<Subject>>(&subjects).unwrap()),
      Err(_) => None,
    }
  }
}
