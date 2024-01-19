use itertools::Itertools;

use crate::data::Timetable;
use crate::filter::{self, Filter};

struct NoCourseOverlapFilter();

pub fn try_parse(spec: &str) -> Option<Box<dyn Filter>> {
  filter::parse_with_key(spec, "no_course_overlap", |_| NoCourseOverlapFilter())
}

impl Filter for NoCourseOverlapFilter {
  fn filter(&self, timetable: &Timetable) -> bool {
    timetable.courses
      .iter()
      .sorted_by_key(|course| course.occurrence.weekday.num_days_from_monday())
      .group_by(|course| course.occurrence.weekday)
      .into_iter()
      .all(|(_, courses)| {
        courses
          .sorted_by_key(|course| course.occurrence.start_time)
          .tuple_windows()
          .all(|(current, next)| next.occurrence.start_time >= current.occurrence.end_time)
      })
  }
}
