use timetable_optimizer_lib::data::Subject;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TimetableGeneratorProps {
  pub subjects: Vec<Subject>,
}

#[function_component(TimetableGenerator)]
pub fn timetable_generator(props: &TimetableGeneratorProps) -> Html {
  let on_restart_agent = {
    let subjects = props.subjects.clone();
    move |_| {
      gloo::console::log!(format!("Restarting agent, there are {} subjects", subjects.len()));
    }
  };

  html! {
    <div>
      { "Timetable Generator" }
      <button onclick={on_restart_agent.clone()}>
        { "Restart agent" }
      </button>
    </div>
  }
}
