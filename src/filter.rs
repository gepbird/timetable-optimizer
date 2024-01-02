use chrono::{Duration, NaiveTime, Weekday};
use itertools::Itertools;

use crate::data::Timetable;

#[derive(Debug)]
pub enum Filter {
  MinStartTime(NaiveTime),
  MaxEndTime(NaiveTime),
  ExcludedWeekDay(Weekday),
  MaxGapBetweenCourses(Duration),
}

impl Filter {
  pub fn parse_filter(spec: &str) -> Self {
    if let Some((key, value)) = spec.split_once('=') {
      match key {
        "min_start_time" => {
          let start_time = NaiveTime::parse_from_str(value, "%H:%M").unwrap();
          return Self::MinStartTime(start_time);
        }
        "max_end_time" => {
          let end_time = NaiveTime::parse_from_str(value, "%H:%M").unwrap();
          return Self::MaxEndTime(end_time);
        }
        "excluded_weekday" => {
          let day = value.parse::<Weekday>().unwrap();
          return Self::ExcludedWeekDay(day);
        }
        "max_gap_minutes_between_courses" => {
          let max_gap = value.parse::<i64>().unwrap();
          return Self::MaxGapBetweenCourses(Duration::minutes(max_gap));
        }
        _ => panic!("Invalid filter key"),
      }
    }

    panic!("Invalid filter specification")
  }

  pub fn filter(&self, timetable: &Timetable) -> bool {
    match self {
      Self::MinStartTime(min_start_time) => timetable
        .iter()
        .all(|course| course.occurence.start_time >= *min_start_time),
      Self::MaxEndTime(max_end_time) => timetable
        .iter()
        .all(|course| course.occurence.end_time <= *max_end_time),
      Self::ExcludedWeekDay(excluded_weekday) => timetable
        .iter()
        .all(|course| course.occurence.week_day != *excluded_weekday),
      Self::MaxGapBetweenCourses(max_gap) => timetable
        .iter()
        .group_by(|course| course.occurence.week_day)
        .into_iter()
        .all(|(_, courses)| {
          courses
            .sorted_by_key(|course| course.occurence.start_time)
            .tuple_windows()
            .all(|(current, next)| {
              next.occurence.start_time - current.occurence.end_time < *max_gap
            })
        }),
    }
  }
}
