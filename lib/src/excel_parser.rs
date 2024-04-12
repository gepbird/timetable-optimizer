use std::{
  io::{BufRead, Seek},
  slice::Iter,
};

use calamine::{Data, DataType as _, Reader, Xlsx};
use chrono::{NaiveTime, Weekday};
use itertools::Itertools;

use crate::data::{Course, CourseType, Enrollment, Occurrence, Subject};

pub fn parse_subject<R: BufRead + Seek>(
  subject_name: String,
  excel: &mut Xlsx<R>,
) -> Subject {
  let sheet = &excel.worksheets()[0].1;
  let courses = sheet
    .rows()
    .skip(1)
    .map(|course| parse_course(subject_name.clone(), course))
    .sorted_by_key(|course| course.course_type)
    .group_by(|course| course.course_type)
    .into_iter()
    .map(|(_type, courses)| courses.collect_vec())
    .collect_vec();
  Subject {
    name: subject_name,
    courses
  }
}

fn parse_course(subject_name: String, row: &[Data]) -> Course {
  let mut r = row.iter();

  let code = cell(&mut r);
  let course_type = parse_course_type(&cell(&mut r));
  let enrollment = parse_enrollment(cell(&mut r));
  r.next();
  r.next();
  let (occurrence, location) = parse_occurrence_and_location(cell(&mut r));
  let teacher = cell(&mut r);
  let language = cell(&mut r);
  let site = cell(&mut r);
  let comment = cell(&mut r);
  let description = cell(&mut r);

  Course::new(
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
  )
}

fn parse_course_type(cell: &str) -> CourseType {
  match cell {
    "Elmélet" => CourseType::Lecture,
    "Labor" => CourseType::Laboratory,
    "Gyakorlat" => CourseType::Practice,
    _ => panic!("Invalid course type: {cell}"),
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
  // occurrence weeks are not stored in the excel file
  // occ_and_loc has even more types of format for PE courses which are not implemented
  if occ_and_loc.is_empty() {
    return (
      Occurrence {
        weeks: None,
        weekday: Weekday::Mon,
        start_time: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        end_time: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
      },
      String::new(),
    );
  }
  let (occ_str, loc) = occ_and_loc.split_once("  ").unwrap();
  let (weekday_str, times_str) = occ_str.split_once(':').unwrap();
  let weekday = parse_weekday(weekday_str);
  let (start_str, end_str) = times_str.split_once('-').unwrap();
  let parse_time = |str| NaiveTime::parse_from_str(str, "%H:%M").unwrap();
  let start_time = parse_time(start_str);
  let end_time = parse_time(end_str);
  (
    Occurrence {
      weeks: None,
      weekday,
      start_time,
      end_time,
    },
    loc.to_string(),
  )
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
    _ => panic!("Invalid weekday: {weekday_str}"),
  }
}

fn cell(row: &mut Iter<'_, Data>) -> String {
  row.next().unwrap().as_string().unwrap()
}
