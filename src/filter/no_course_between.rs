use chrono::{NaiveTime, Weekday};

use crate::data::Timetable;
use crate::filter::{self, Filter};

struct NoCourseBetweenFilter {
  pub weekday: Weekday,
  pub start: NaiveTime,
  pub end: NaiveTime,
}

pub fn try_parse(spec: &str) -> Option<Box<dyn Filter>> {
  filter::parse_with_key(spec, "no_course_between", |value| {
    let mut tokens = value.split(',');
    NoCourseBetweenFilter {
      weekday: tokens.next().unwrap().parse::<Weekday>().unwrap(),
      start: NaiveTime::parse_from_str(tokens.next().unwrap(), "%H:%M").unwrap(),
      end: NaiveTime::parse_from_str(tokens.next().unwrap(), "%H:%M").unwrap(),
    }
  })
}

impl Filter for NoCourseBetweenFilter {
  fn filter(&self, timetable: &Timetable) -> bool {
    timetable
      .courses
      .iter()
      .map(|course| &course.occurrence)
      .filter(|occ| occ.weekday == self.weekday)
      .all(|occ| occ.end_time <= self.start || occ.start_time >= self.end)
  }
}
