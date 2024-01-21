use chrono::Duration;
use itertools::Itertools;

use crate::data::Timetable;
use crate::filter::{self, Filter};

struct MaxGapBetweenCoursesFilter(Duration);

pub fn try_parse(spec: &str) -> Option<Result<Box<dyn Filter>, String>> {
  filter::parse_with_key(spec, "max_gap_minutes_between_courses", |value| {
    let max_gap = value.parse::<i64>().map_err(|_| format!("Invalid positive number: {value}"))?;
    Ok(MaxGapBetweenCoursesFilter(Duration::minutes(max_gap)))
  })
}

impl Filter for MaxGapBetweenCoursesFilter {
  fn filter(&self, timetable: &Timetable) -> bool {
    timetable
      .courses
      .iter()
      .sorted_by_key(|course| course.occurrence.weekday.num_days_from_monday())
      .group_by(|course| course.occurrence.weekday)
      .into_iter()
      .all(|(_, courses)| {
        courses
          .sorted_by_key(|course| course.occurrence.start_time)
          .tuple_windows()
          .all(|(current, next)| next.occurrence.start_time - current.occurrence.end_time < self.0)
      })
  }
}
