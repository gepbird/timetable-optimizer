use std::fs;

use crate::data::Subject;

pub fn get_subjects() -> Vec<Subject> {
  let json = fs::read_to_string("data/sample.json").unwrap();
  serde_json::from_str(&json).unwrap()
}
