use std::io::{self, Write};

use crate::data::Timetable;

pub mod exclude_teacher;
pub mod excluded_weekday;
pub mod max_end_time;
pub mod max_gap_between_courses;
pub mod min_start_time;
pub mod no_course_between;
pub mod no_course_overlap;

pub trait Filter {
  fn filter(&self, timetable: &crate::data::Timetable) -> bool;
}

fn parse_with_key<F, T>(spec: &str, key: &str, parse_fn: F) -> Option<Box<dyn Filter>>
where
  F: FnOnce(&str) -> T,
  T: Filter + 'static,
{
  match spec.strip_prefix(&(key.to_string() + "=")) {
    Some(value) => Some(Box::new(parse_fn(value))),
    None => None,
  }
}

fn parse_filter(spec: &str) -> Result<Box<dyn Filter>, String> {
  let parsers: &[fn(&str) -> Option<Box<dyn Filter>>] = &[
    min_start_time::try_parse,
    max_end_time::try_parse,
    excluded_weekday::try_parse,
    max_gap_between_courses::try_parse,
    exclude_teacher::try_parse,
    no_course_overlap::try_parse,
    no_course_between::try_parse,
  ];

  parsers
    .into_iter()
    .find_map(|parser| parser(spec))
    .ok_or_else(|| format!("Invalid filter specification: {spec}"))
}

pub fn prompt_filters() -> Vec<Box<dyn Filter>> {
  print!("Enter filter: ");
  io::stdout().flush().unwrap();
  let mut specs = String::new();
  io::stdin().read_line(&mut specs).unwrap();

  let filters_parsed = specs
    .trim()
    .split(' ')
    .filter(|spec| !spec.is_empty())
    .map(|spec| parse_filter(spec))
    .collect();

  match filters_parsed {
    Ok(filters) => filters,
    Err(e) => {
      eprintln!("Error parsing filter: {e}");
      prompt_filters()
    }
  }
}

pub fn filter_timetables(
  timetables: Vec<Timetable>,
  filters: Vec<Box<dyn Filter>>,
) -> Vec<Timetable> {
  timetables
    .into_iter()
    .filter(|timetable| filters.iter().all(|filter| filter.filter(timetable)))
    .collect()
}
