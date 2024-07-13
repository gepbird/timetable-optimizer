use std::rc::Rc;

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(UploadComponent)]
pub fn upload_component() -> Html {
  let readers = use_state(Vec::new);
  let processed_files: UseStateHandle<Vec<Vec<u8>>> = use_state(Vec::new);
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

  use_effect_with(queued_files, move |queued_files| {
    let queued_files = queued_files.clone();
    let mut new_queued_files = (*queued_files).clone();
    let queued_file = new_queued_files.pop();

    match queued_file {
      None => {
        if processed_files.len() > 0 {
          gloo::console::log!("processed: ", processed_files.len());
        }
      }
      Some(queued_file) => {
        let reader = Rc::new(gloo::file::callbacks::read_as_bytes(
          &queued_file,
          move |bytes| {
            let bytes = bytes.unwrap();
            processed_files.set({
              let mut processed_files = (*processed_files).clone();
              processed_files.push(bytes);
              processed_files
            });
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
