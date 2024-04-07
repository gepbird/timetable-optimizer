use std::{
  io::{BufRead, Seek},
  slice::Iter,
};

use calamine::{DataType, Reader, Xlsx};
use chrono::{NaiveTime, Weekday};
use itertools::Itertools;

use crate::data::{Course, CourseType, Enrollment, Occurrence, Subject};

pub fn parse_subjects<R: BufRead + Seek>(excel: &mut Xlsx<R>) -> Vec<Subject> {
  let sheet = &excel.worksheets()[0].1;
  let subjects = sheet
    .rows()
    .into_iter()
    .skip(1)
    .map(parse_subject)
    .collect_vec();
  subjects
}

pub fn parse_courses<R: BufRead + Seek>(subject_name: &str, excel: &mut Xlsx<R>) -> Vec<Course> {
  let sheet = &excel.worksheets()[0].1;
  let courses = sheet
    .rows()
    .into_iter()
    .skip(1)
    .map(|course| parse_course(subject_name.to_string(), course))
    .collect_vec();
  courses
}

fn parse_subject(row: &[DataType]) -> Subject {
  let mut r = row.iter();

  let name = cell(&mut r);
  let courses = Vec::default();
  let code = cell(&mut r);
  let group_name = cell_opt(&mut r);
  let number = cell_num_opt(&mut r);
  let recommended_semester = cell_num_opt(&mut r);
  let credits = cell_num(&mut r);
  let subject_type = cell_opt(&mut r);
  let comment = cell_opt(&mut r);
  let completed = cell_bool(&mut r);
  let enrolled = cell_bool(&mut r);
  let queue = cell_opt(&mut r);

  Subject {
    name,
    code,
    group_name,
    number,
    recommended_semester,
    credits,
    subject_type,
    comment,
    completed,
    enrolled,
    queue,
    courses,
  }
}

fn parse_course(subject_name: String, row: &[DataType]) -> Course {
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
  return (
    Occurrence {
      weeks: None,
      weekday,
      start_time,
      end_time,
    },
    loc.to_string(),
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
    _ => panic!("Invalid weekday: {weekday_str}"),
  }
}

fn cell(row: &mut Iter<'_, DataType>) -> String {
  row.next().unwrap().as_string().unwrap()
}

fn cell_opt(row: &mut Iter<'_, DataType>) -> Option<String> {
  let c = cell(row);
  if c.is_empty() {
    None
  } else {
    Some(c)
  }
}

fn cell_bool(row: &mut Iter<'_, DataType>) -> bool {
  let value = cell(row);
  match value.as_str() {
    "Nem" => false,
    "Igen" => true,
    _ => panic!("Invalid boolean value {value}"),
  }
}

fn cell_num(row: &mut Iter<'_, DataType>) -> u32 {
  cell(row).parse::<u32>().unwrap()
}

fn cell_num_opt(row: &mut Iter<'_, DataType>) -> Option<u32> {
  cell_opt(row).map(|n| n.parse::<u32>().unwrap())
}
