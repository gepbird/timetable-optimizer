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
  subjects: Vec<Subject>,
}

pub enum Msg {
  CoursesUploaded(Vec<gloo::file::File>),
  CourseProcessed(String, Vec<u8>),
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
        let subject = Subject {
          name: file_name.clone(),
          courses: excel_parser::parse_courses(file_name.as_str(), &mut excel),
        };
        self.subjects.push(subject);
        self.save_subjects();
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
        { self.view_all_courses() }
        { self.view_stats() }
      </main>
    }
  }
}
impl App {
  fn view_all_courses(&self) -> Html {
    html! {
      { for self.subjects.iter().map(|s| {
        html! {
          <div class="my-6">
            <h2>{ &s.name }</h2>
            { self.view_courses(&s.courses) }
          </div>
        }
      }) }
    }
  }

  fn view_courses(&self, courses: &Vec<Vec<Course>>) -> Html {
    let courses: Vec<&Course> = courses.iter().flatten().collect();
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
      <>
        <h1>{ "Statistics" }</h1>
        <p>{ format!("Total courses inputted: {}", stats::count_all_courses(&self.subjects)) }</p>
        <p>{ format!("Total courses in a timetable: {}", stats::count_course_per_timetable(&self.subjects)) }</p>
        <p>{ format!("Total possible timetables: {}", stats::count_all_timetables(&self.subjects)) }</p>
      </>
    }
  }

  fn save_subjects(&self) {
    let subjects = serde_json::to_string(&self.subjects).unwrap();
    LocalStorage::set("subjects", subjects).unwrap();
  }

  fn load_subjects() -> Vec<Subject> {
    match LocalStorage::get::<String>("subjects") {
      Ok(subjects) => serde_json::from_str::<Vec<Subject>>(&subjects).unwrap(),
      Err(_) => vec![],
    }
  }
}
