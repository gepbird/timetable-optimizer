use std::collections::HashMap;

use gloo::console;
use gloo::file::callbacks::FileReader;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct App {
  readers: HashMap<String, FileReader>,
}

pub enum Msg {
  SubjectUploaded(gloo::file::File),
  SubjectProcessed(String, String),
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    App {
      readers: HashMap::default(),
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::SubjectUploaded(file) => {
        let file_name = file.name();
        let link = ctx.link().clone();
        let reader = gloo::file::callbacks::read_as_text(&file, move |text| {
          link.send_message(Msg::SubjectProcessed(file_name, text.unwrap()));
        });
        self.readers.insert(file.name(), reader);
        false
      }
      Msg::SubjectProcessed(file_name, text) => {
        self.readers.remove(&file_name);
        console::log!(text);
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
      </main>
    }
  }
}
