use std::{fs::File, io::Write};

use itertools::Itertools;

use crate::data::Timetable;

pub fn save_course_codes(timetable: &Timetable, file_path: String) {
  let course_codes = timetable.courses
    .iter()
    .map(|course| course.code.clone())
    .join("\n");

  File::create(file_path)
    .unwrap()
    .write_all(course_codes.as_bytes())
    .unwrap();
}
