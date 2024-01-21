use crate::data::Timetable;
use crate::filter::{self, Filter};

struct ExcludeTeacherFilter(String);

pub fn try_parse(spec: &str) -> Option<Result<Box<dyn Filter>, String>> {
  filter::parse_with_key(spec, "exclude_teacher", |value| {
    Ok(ExcludeTeacherFilter(value.to_string()))
  })
}

impl Filter for ExcludeTeacherFilter {
  fn filter(&self, timetable: &Timetable) -> bool {
    timetable
      .courses
      .iter()
      .all(|course| !course.teacher.contains(&self.0))
  }
}
