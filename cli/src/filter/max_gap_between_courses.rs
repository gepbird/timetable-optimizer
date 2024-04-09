use chrono::Duration;
use itertools::Itertools;

use crate::filter::{self, Filter};
use timetable_optimizer_lib::data::Timetable;

struct MaxGapBetweenCoursesFilter(Duration);

pub fn try_parse(spec: &str) -> Option<Result<Box<dyn Filter>, String>> {
  filter::parse_with_key(spec, "max_gap_minutes_between_courses", |value| {
    let max_gap = value
      .parse::<i64>()
      .map_err(|_| format!("Invalid positive number: {value}"))?;
    Ok(MaxGapBetweenCoursesFilter(Duration::minutes(max_gap)))
  })
}

impl Filter for MaxGapBetweenCoursesFilter {
  fn filter(&self, timetable: &Timetable) -> bool {
    timetable
      .courses
      .iter()
      .group_by(|course| course.occurrence.weekday)
      .into_iter()
      .all(|(_, courses)| {
        courses
          .tuple_windows()
          .all(|(current, next)| next.occurrence.start_time - current.occurrence.end_time < self.0)
      })
  }
}
