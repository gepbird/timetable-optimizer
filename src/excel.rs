use std::{fs, slice::Iter};

use calamine::{DataType, Reader, Xlsx};
use chrono::{NaiveTime, Weekday};
use itertools::Itertools;

use crate::data::{Course, CourseType, Enrollment, Occurrence, Weeks};

// an internal tool that converts Neptun's exported .xlsx into a .json
// manual adjustions after json generation:
// - categorize courses into one-of-courses
// - fill out occurrence.weeks and subject_name, default is just a placeholder
#[allow(dead_code)]
pub fn process(filename: &str) {
  let courses = parse_courses(&format!("data/{filename}.xlsx"));
  let json = serde_json::to_string_pretty(&courses).unwrap();
  fs::write(&format!("data/{filename}.json"), json).unwrap();
}

fn parse_courses(path: &str) -> Vec<Course> {
  let mut excel: Xlsx<_> = calamine::open_workbook(path).unwrap();
  let sheet = &excel.worksheets()[0].1;
  let courses = sheet
    .rows()
    .into_iter()
    .skip(1)
    .map(parse_course)
    .collect_vec();
  courses
}

fn parse_course(row: &[DataType]) -> Course {
  let mut row_iter = row.iter();
  let cell = |r: &mut Iter<'_, DataType>| r.next().unwrap().as_string().unwrap();

  let subject_name = "Placeholder subject".to_owned();
  let code = cell(&mut row_iter);
  let course_type_str = cell(&mut row_iter);
  let course_type: CourseType = serde_json::from_str(&format!("\"{course_type_str}\"")).unwrap();
  let enrollment = parse_enrollment(cell(&mut row_iter));
  row_iter.next();
  row_iter.next();
  let (occurrence, location) = parse_occurrence_and_location(cell(&mut row_iter));
  let teacher = cell(&mut row_iter);
  let language = cell(&mut row_iter);
  let site = cell(&mut row_iter);
  let comment = cell(&mut row_iter);
  let description = cell(&mut row_iter);

  Course {
    subject_name,
    code,
    course_type,
    enrollment,
    location,
    teacher,
    language,
    site,
    comment,
    description,
    occurrence,
  }
}

fn parse_enrollment(cell: String) -> Enrollment {
  let mut tokens = cell.split('/');
  let mut parse_next = || tokens.next().unwrap().parse().unwrap();
  Enrollment {
    people_joined: parse_next(),
    people_queue: parse_next(),
    people_limit: parse_next(),
  }
}

fn parse_occurrence_and_location(occ_and_loc: String) -> (Occurrence, String) {
  let (occ_str, loc) = occ_and_loc.split_once("  ").unwrap();
  let (weekday_str, times_str) = occ_str.split_once(':').unwrap();
  let weekday = parse_weekday(weekday_str);
  let (start_str, end_str) = times_str.split_once('-').unwrap();
  let parse_time = |str| NaiveTime::parse_from_str(str, "%H:%M").unwrap();
  let start_time = parse_time(start_str);
  let end_time = parse_time(end_str);
  return (
    Occurrence {
      weeks: Weeks::Every, // placeholder, input data doesn't contain it
      weekday,
      start_time,
      end_time,
    },
    loc.to_owned(),
  );
}

fn parse_weekday(weekday_str: &str) -> Weekday {
  match weekday_str {
    "H" => Weekday::Mon,
    "K" => Weekday::Tue,
    "SZE" => Weekday::Wed,
    "CS" => Weekday::Thu,
    "P" => Weekday::Fri,
    "SZO" => Weekday::Sat,
    "V" => Weekday::Sun,
    _ => panic!("Invalid weekday: {}", weekday_str),
  }
}
