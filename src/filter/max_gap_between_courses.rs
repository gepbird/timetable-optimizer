use chrono::Duration;
use itertools::Itertools;

use crate::data::Timetable;
use crate::filter::{self, Filter};

struct MaxGapBetweenCoursesFilter(Duration);

pub fn try_parse(spec: &str) -> Option<Box<dyn Filter>> {
  filter::parse_with_key(spec, "max_gap_minutes_between_courses", |value| {
    let max_gap = value.parse::<i64>().unwrap();
    MaxGapBetweenCoursesFilter(Duration::minutes(max_gap))
  })
}

impl Filter for MaxGapBetweenCoursesFilter {
  fn filter(&self, timetable: &Timetable) -> bool {
    timetable
      .iter()
      .group_by(|course| course.occurence.week_day)
      .into_iter()
      .all(|(_, courses)| {
        courses
          .sorted_by_key(|course| course.occurence.start_time)
          .tuple_windows()
          .all(|(current, next)| next.occurence.start_time - current.occurence.end_time < self.0)
      })
  }
}
