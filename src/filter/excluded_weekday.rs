use chrono::Weekday;

use crate::data::Timetable;
use crate::filter::{self, Filter};

struct ExcludedWeekDayFilter(Weekday);

pub fn try_parse(spec: &str) -> Option<Box<dyn Filter>> {
  filter::parse_with_key(spec, "excluded_weekday", |value| {
    let day = value.parse::<Weekday>().unwrap();
    ExcludedWeekDayFilter(day)
  })
}

impl Filter for ExcludedWeekDayFilter {
  fn filter(&self, timetable: &Timetable) -> bool {
    timetable
      .iter()
      .all(|course| course.occurence.week_day != self.0)
  }
}
