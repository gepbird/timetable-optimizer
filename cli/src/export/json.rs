use std::{fs::File, io::Write, path::PathBuf};

use crate::data::Timetable;

pub fn save_timetable_json(timetable: &Timetable, file_path: PathBuf) {
  let serialized_timetable = serde_json::to_string_pretty(timetable).unwrap();

  File::create(file_path)
    .unwrap()
    .write_all(serialized_timetable.as_bytes())
    .unwrap();
}
