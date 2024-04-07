use std::collections::HashMap;
use std::io::Cursor;

use calamine::Xlsx;
use gloo::file::callbacks::FileReader;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use timetable_optimizer_lib::data::Subject;
use timetable_optimizer_lib::excel_parser;

pub struct App {
  readers: HashMap<String, FileReader>,
  subjects: Option<Vec<Subject>>,
}

pub enum Msg {
  SubjectUploaded(gloo::file::File),
  SubjectProcessed(String, Vec<u8>),
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    App {
      readers: HashMap::default(),
      subjects: None,
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
              <th>{ "Code" }</th>
              <th>{ "Recommended semester" }</th>
              <th>{ "Credits" }</th>
            </tr>
          </thead>
          <tbody>
            { for subjects.iter().map(|s| html! {
              <tr>
                <td>{ &s.name }</td>
                <td>{ &s.code }</td>
                <td>{ &s.recommended_semester.map_or("N/A".to_owned(), |v| v.to_string()) }</td>
                <td>{ s.credits }</td>
              </tr>
            }) }
          </tbody>
        </table>
      }
    }
  }
}
