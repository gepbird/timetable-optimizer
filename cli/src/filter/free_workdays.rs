use chrono::Weekday;
use itertools::Itertools;

use timetable_optimizer_lib::data::Timetable;
use crate::filter::{self, Filter};

struct FreeWorkdays(usize);

pub fn try_parse(spec: &str) -> Option<Result<Box<dyn Filter>, String>> {
  filter::parse_with_key(spec, "free_workdays", |value| {
    let days = value.parse::<usize>().map_err(|_| format!("Invalid positive number: {value}"))?;
    Ok(FreeWorkdays(days))
  })
}

impl Filter for FreeWorkdays {
  fn filter(&self, timetable: &Timetable) -> bool {
    let workdays = &[
      Weekday::Mon,
      Weekday::Tue,
      Weekday::Wed,
      Weekday::Thu,
      Weekday::Fri,
    ];

    timetable.courses
      .iter()
      .group_by(|timetable| timetable.occurrence.weekday)
      .into_iter()
      .filter(|(weekday, _courses)| workdays.contains(weekday))
      .count() <= workdays.len() - self.0
  }
}
