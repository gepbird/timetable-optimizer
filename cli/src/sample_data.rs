use std::fs;

use timetable_optimizer_lib::data::Subject;

pub fn get_subjects() -> Vec<Subject> {
  let json = fs::read_to_string("data/sample.json").unwrap();
  let mut subjects: Vec<Subject> = serde_json::from_str(&json).unwrap();

  subjects.iter_mut().for_each(|subject| {
    subject.courses.iter_mut().for_each(|one_of_courses| {
      one_of_courses
        .iter_mut()
        .for_each(|course| course.update_hash())
    })
  });

  subjects
}
