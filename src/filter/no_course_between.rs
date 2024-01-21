use chrono::{NaiveTime, Weekday};

use crate::data::Timetable;
use crate::filter::{self, Filter};

struct NoCourseBetweenFilter {
  pub weekday: Weekday,
  pub start: NaiveTime,
  pub end: NaiveTime,
}

pub fn try_parse(spec: &str) -> Option<Result<Box<dyn Filter>, String>> {
  filter::parse_with_key(spec, "no_course_between", |value| {
    let mut tokens = value.split(',');
    let mut token = || {
      tokens
        .next()
        .ok_or(format!("Not enough comma seperated arguments: {value}"))
    };
    let weekday = token()?;
    Ok(NoCourseBetweenFilter {
      weekday: weekday.parse::<Weekday>().map_err(|_| format!("Invalid weekday: {weekday}"))?,
      start: NaiveTime::parse_from_str(token()?, "%H:%M")
        .map_err(|_| format!("Invalid time: {value}"))?,
      end: NaiveTime::parse_from_str(token()?, "%H:%M")
        .map_err(|_| format!("Invalid time: {value}"))?,
    })
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
