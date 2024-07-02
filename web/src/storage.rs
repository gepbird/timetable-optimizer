use gloo::storage::{LocalStorage, Storage};
use timetable_optimizer_lib::data::Subject;

pub fn save_subjects(subjects: &Vec<Subject>) {
  let subjects = serde_json::to_string(subjects).unwrap();
  LocalStorage::set("subjects", subjects).unwrap();
}

pub fn load_subjects() -> Vec<Subject> {
  match LocalStorage::get::<String>("subjects") {
    Ok(subjects) => serde_json::from_str::<Vec<Subject>>(&subjects).unwrap(),
    Err(_) => vec![],
  }
}
