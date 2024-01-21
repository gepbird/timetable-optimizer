use std::{
  fs::File,
  io::{self, BufReader, Write},
  slice::Iter,
};

use calamine::{DataType, Reader, Xlsx};
use itertools::Itertools;

use crate::data::Subject;

pub fn import_subjects() -> Vec<Subject> {
  println!("Started importing subjects and courses");
  println!("When prompted, export the appropriate data from Neptun to an Excel file and drag and drop the file here");
  println!("Leave the prompt empty to skip importing that data");

  let subjects = match read_xlsx("subjects") {
    Some(mut excel) => {
      let worksheets = excel.worksheets();
      let (_name, sheet) = worksheets.first().unwrap();
      sheet
        .rows()
        .into_iter()
        .skip(1)
        .map(parse_subject)
        .collect_vec()
    }
    None => vec![],
  };

  let subject_count = subjects.len();
  println!("Parsed and loaded {subject_count} subjects");

  subjects
}

fn parse_subject(row: &[DataType]) -> Subject {
  let mut r = row.iter();

  let name = cell(&mut r);
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
  let courses = vec![];

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

fn read_xlsx<'a>(data_name: &str) -> Option<Xlsx<BufReader<File>>> {
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
