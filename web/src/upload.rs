use std::{io::Cursor, rc::Rc};

use calamine::Xlsx;
use timetable_optimizer_lib::{data::Subject, excel_parser};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub on_files_processed: Callback<Vec<Subject>>,
}

#[function_component(UploadComponent)]
pub fn upload_component(props: &Props) -> Html {
  let readers = use_state(Vec::new);
  let processed_files = use_state(Vec::new);
  let queued_files = use_state(Vec::new);

  let on_file_change = {
    let queued_files = queued_files.clone();
    Callback::from(move |e: Event| {
      let input: HtmlInputElement = e.target_unchecked_into();
      let files = input.files().unwrap();
      queued_files.set(
        js_sys::try_iter(&files)
          .unwrap()
          .unwrap()
          .map(|file| file.unwrap())
          .map(web_sys::File::from)
          .map(gloo::file::File::from)
          .collect(),
      );
    })
  };

  let on_files_processed = props.on_files_processed.clone();
  use_effect_with(queued_files, move |queued_files| {
    let queued_files = queued_files.clone();
    let mut new_queued_files = (*queued_files).clone();
    let queued_file = new_queued_files.pop();

    match queued_file {
      None => {
        if processed_files.len() > 0 {
          on_files_processed.emit((*processed_files).clone());
          processed_files.set(Vec::new());
          readers.set(Vec::new());
        }
      }
      Some(queued_file) => {
        let file_name = queued_file.name();
        let reader = Rc::new(gloo::file::callbacks::read_as_bytes(
          &queued_file,
          move |bytes| {
            match bytes {
              Err(err) => gloo::dialogs::alert(&err.to_string()),
              Ok(bytes) => {
                let subject = parse_subject(file_name, bytes);
                processed_files.set({
                  let mut processed_files = (*processed_files).clone();
                  processed_files.push(subject);
                  processed_files
                });
              }
            }
            queued_files.set(new_queued_files);
          },
        ));
        readers.set({
          let mut readers = (*readers).clone();
          readers.push(reader);
          readers
        })
      }
    }
  });

  html! {
    <div>
      <input type="file" multiple=true onchange={on_file_change} />
    </div>
  }
}

fn parse_subject(file_name: String, bytes: Vec<u8>) -> Subject {
  let cursor = Cursor::new(bytes);
  let mut excel: Xlsx<_> = calamine::open_workbook_from_rs(cursor).unwrap();
  excel_parser::parse_subject(file_name, &mut excel)
}
