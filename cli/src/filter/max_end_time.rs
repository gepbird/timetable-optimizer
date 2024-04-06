use chrono::NaiveTime;

use timetable_optimizer_lib::data::Timetable;
use crate::filter::{self, Filter};

struct MaxEndTimeFilter(NaiveTime);

pub fn try_parse(spec: &str) -> Option<Result<Box<dyn Filter>, String>> {
  filter::parse_with_key(spec, "max_end_time", |value| {
    let end_time =
      NaiveTime::parse_from_str(value, "%H:%M").map_err(|_| format!("Invalid time: {value}"))?;
    Ok(MaxEndTimeFilter(end_time))
  })
}

impl Filter for MaxEndTimeFilter {
  fn filter(&self, timetable: &Timetable) -> bool {
    timetable
      .courses
      .iter()
      .all(|course| course.occurrence.end_time <= self.0)
  }
}
