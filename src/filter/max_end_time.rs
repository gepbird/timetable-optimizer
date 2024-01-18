use chrono::NaiveTime;

use crate::data::Timetable;
use crate::filter::{self, Filter};

struct MaxEndTimeFilter(NaiveTime);

pub fn try_parse(spec: &str) -> Option<Box<dyn Filter>> {
  filter::parse_with_key(spec, "max_end_time", |value| {
    let end_time = NaiveTime::parse_from_str(value, "%H:%M").unwrap();
    MaxEndTimeFilter(end_time)
  })
}

impl Filter for MaxEndTimeFilter {
  fn filter(&self, timetable: &Timetable) -> bool {
    timetable.courses
      .iter()
      .all(|course| course.occurrence.end_time <= self.0)
  }
}
