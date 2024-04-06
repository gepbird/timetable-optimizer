use std::{
  fs::File,
  io::{self, BufReader, Write},
};

use calamine::Xlsx;
use itertools::Itertools;

use timetable_optimizer_lib::{
  data::{OneOfCourse, Subject},
  excel_parser,
};

pub fn setup() -> Vec<Subject> {
  let mut subjects = import_subjects();
  for subject in subjects.iter_mut() {
    if let Some(courses) = import_courses(&subject.name) {
      subject.courses = courses;
    }
  }
  subjects
}

pub fn import_subjects() -> Vec<Subject> {
  println!("Started importing subjects and courses");
  println!("When prompted, export the appropriate data from Neptun to an Excel file and drag and drop the file here");
  println!("Leave the prompt empty to skip importing that data");
  println!("Don't import PE courses, use the no_course_between filter instead");

  let subjects = match read_xlsx("subjects") {
    Some(mut excel) => excel_parser::parse_subjects(&mut excel),
    None => vec![],
  };

  let subject_count = subjects.len();
  println!("Parsed and loaded {subject_count} subjects");

  subjects
}

fn import_courses(subject_name: &str) -> Option<Vec<OneOfCourse>> {
  read_xlsx(&format!("{subject_name} courses")).map(|mut excel| {
    let courses = excel_parser::parse_courses(subject_name, &mut excel);
    courses
      .into_iter()
      .sorted_by_key(|course| course.course_type)
      .group_by(|course| course.course_type)
      .into_iter()
      .map(|(_type, courses)| courses.collect_vec())
      .collect_vec()
  })
}

fn read_xlsx<'a>(data_name: &str) -> Option<Xlsx<BufReader<File>>> {
  print!("Enter {data_name}: ");
  io::stdout().flush().unwrap();
  let mut dirty_path = String::new();
  io::stdin().read_line(&mut dirty_path).unwrap();
  let path = dirty_path.trim().trim_matches('\'');
  if path.is_empty() {
    return None;
  }

  let excel: Xlsx<_> = match calamine::open_workbook(path) {
    Ok(excel) => excel,
    Err(err) => {
      eprintln!("Failed to open excel file: {}", err);
      return read_xlsx(data_name);
    }
  };

  Some(excel)
}
