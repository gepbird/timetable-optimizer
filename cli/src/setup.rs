use std::{
  io::{self, Write},
  path::Path,
};

use calamine::Xlsx;

use timetable_optimizer_lib::{data::Subject, excel_parser};

pub fn setup() -> Vec<Subject> {
  println!("Export the courses from each subject from Neptun to an Excel file then drag and drop those file here");
  println!("Leave the prompt empty to finish importing");
  println!("Don't import PE courses, use the no_course_between filter instead");

  let mut subjects = vec![];
  while let Some(subject) = read_subject() {
    subjects.push(subject);
  }

  let subject_count = subjects.len();
  println!("Parsed and loaded {subject_count} subjects");

  subjects
}

fn read_subject() -> Option<Subject> {
  print!("Enter courses for a subject: ");
  io::stdout().flush().unwrap();
  let mut dirty_path = String::new();
  io::stdin().read_line(&mut dirty_path).unwrap();
  let path_str = dirty_path.trim().trim_matches('\'');
  if path_str.is_empty() {
    return None;
  }

  let mut excel: Xlsx<_> = match calamine::open_workbook(path_str) {
    Ok(excel) => excel,
    Err(err) => {
      eprintln!("Failed to open excel file: {}", err);
      return read_subject();
    }
  };

  let path = Path::new(path_str);
  let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
  let courses = excel_parser::parse_courses(&file_name, &mut excel);
  let subject = Subject {
    name: file_name,
    courses,
  };
  Some(subject)
}
