use std::{
  fs::File,
  io::{self, BufReader, Write},
  slice::Iter,
};

use calamine::{DataType, Reader, Xlsx};
use itertools::Itertools;

use crate::{
  data::{Course, CourseType, OneOfCourse, Subject},
  excel,
};

pub fn import_subjects() -> Vec<Subject> {
  println!("Started importing subjects and courses");
  println!("When prompted, export the appropriate data from Neptun to an Excel file and drag and drop the file here");
  println!("Leave the prompt empty to skip importing that data");
  println!("Don't import PE courses, use the no_course_between filter instead");

  let subjects = match read_xlsx("subjects".to_owned()) {
    Some(mut excel) => {
      let worksheets = excel.worksheets();
      let (_name, sheet) = worksheets.first().unwrap();
      sheet
        .rows()
        .into_iter()
        .skip(1)
        .map(parse_subject)
        .flatten()
        .collect_vec()
    }
    None => vec![],
  };

  let subject_count = subjects.len();
  println!("Parsed and loaded {subject_count} subjects");

  subjects
}

fn import_courses(subject_name: &str) -> Option<Vec<OneOfCourse>> {
  read_xlsx(format!("{subject_name} courses")).map(|mut excel| {
    let worksheets = excel.worksheets();
    let (_name, sheet) = worksheets.first().unwrap();
    let courses = sheet
      .rows()
      .into_iter()
      .skip(1)
      .map(|row| parse_course(subject_name.to_owned(), row))
      .sorted_by_key(|course| course.course_type)
      .group_by(|course| course.course_type)
      .into_iter()
      .map(|(_type, courses)| courses.collect_vec())
      .collect_vec();
    courses
  })
}

fn parse_subject(row: &[DataType]) -> Option<Subject> {
  let mut r = row.iter();

  let name = cell(&mut r);
  let courses = match import_courses(&name) {
    Some(courses) => courses,
    None => return None,
  };

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

  Some(Subject {
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
  })
}

fn parse_course(subject_name: String, row: &[DataType]) -> Course {
  let mut r = row.iter();

  let code = cell(&mut r);
  let course_type_str = cell(&mut r);
  let course_type: CourseType = serde_json::from_str(&format!("\"{course_type_str}\"")).unwrap();
  let enrollment = excel::parse_enrollment(cell(&mut r));
  r.next();
  r.next();
  let (occurrence, location) = excel::parse_occurrence_and_location(cell(&mut r));
  let teacher = cell(&mut r);
  let language = cell(&mut r);
  let site = cell(&mut r);
  let comment = cell(&mut r);
  let description = cell(&mut r);

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

fn read_xlsx<'a>(data_name: String) -> Option<Xlsx<BufReader<File>>> {
  print!("Enter {data_name}: ");
  let mut dirty_path = String::new();
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut dirty_path).unwrap();
  let path = dirty_path.trim().trim_matches('\'');
  if path.is_empty() {
    return None;
  }

  let excel: Xlsx<_> = match calamine::open_workbook(path) {
    Ok(excel) => excel,
    Err(err) => {
      println!("Failed to open excel file: {}", err);
      return read_xlsx(data_name);
    }
  };

  Some(excel)
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
