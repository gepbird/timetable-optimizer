use chrono::Weekday;

use crate::data::Timetable;
use crate::filter::{self, Filter};

struct FreeWorkday();

pub fn try_parse(spec: &str) -> Option<Box<dyn Filter>> {
  filter::parse_with_key(spec, "free_workday", |_| FreeWorkday())
}

impl Filter for FreeWorkday {
  fn filter(&self, timetable: &Timetable) -> bool {
    let workdays = &[
      Weekday::Mon,
      Weekday::Tue,
      Weekday::Wed,
      Weekday::Thu,
      Weekday::Fri,
    ];
    workdays.into_iter().any(|workday| {
      timetable
        .courses
        .iter()
        .all(|course| course.occurrence.weekday != *workday)
    })
  }
}
