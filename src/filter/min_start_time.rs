use chrono::NaiveTime;

use crate::data::Timetable;
use crate::filter::{self, Filter};

struct MinStartTimeFilter(NaiveTime);

pub fn try_parse(spec: &str) -> Option<Box<dyn Filter>> {
  filter::parse_with_key(spec, "min_start_time", |value| {
    let start_time = NaiveTime::parse_from_str(value, "%H:%M").unwrap();
    MinStartTimeFilter(start_time)
  })
}

impl Filter for MinStartTimeFilter {
  fn filter(&self, timetable: &Timetable) -> bool {
    timetable
      .courses
      .iter()
      .all(|course| course.occurrence.start_time >= self.0)
  }
}
