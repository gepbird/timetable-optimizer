use std::fs::{self, File};
use std::io::{self, Write};

use data::{Course, Subject, Timetable};
use itertools::Itertools;
use permutator::CartesianProduct;

mod data;
mod filter;
mod sample_data;

fn save_timetables(timetables: Vec<Vec<&Course>>) {
  let out_dir = "out";
  let full_dir = format!("{out_dir}/full");
  let codes_dir = format!("{out_dir}/course-codes");
  fs::create_dir_all(out_dir).unwrap();

  for subdirectory in vec![&full_dir, &codes_dir]{
    fs::create_dir_all(&subdirectory).ok();
    for entry in fs::read_dir(subdirectory).unwrap() {
      fs::remove_file(entry.unwrap().path()).unwrap();
    }
  }

  for (index, timetable) in timetables.iter().enumerate() {
    let serialized_timetable = serde_json::to_string_pretty(timetable).unwrap();
    File::create(format!("{full_dir}/timetable_{:04}.json", index))
      .unwrap()
      .write_all(serialized_timetable.as_bytes())
      .unwrap();

    let course_codes = timetable
      .into_iter()
      .map(|course| course.code.clone())
      .join("\n");
    File::create(format!("{codes_dir}/timetable_{:04}.txt", index))
      .unwrap()
      .write_all(course_codes.as_bytes())
      .unwrap();
  }
}

pub fn generate_timetables<'a>(subjects: &'a Vec<Subject>) -> Vec<Timetable<'a>> {
  let one_of_courses: Vec<Vec<&'a Course>> = subjects
    .iter()
    .flat_map(|subject| &subject.courses)
    .map(|one_of_course| one_of_course.iter().collect::<Vec<&'a Course>>())
    .collect();

  let timetables: Vec<Vec<&'a Course>> = one_of_courses
    .iter()
    .map(|x| x.as_slice())
    .collect::<Vec<&[&'a Course]>>()
    .cart_prod()
    .map(|cp| {
      cp.into_iter()
        .map(|&course| course)
        .collect::<Vec<&'a Course>>()
    })
    .collect();

  timetables
}

fn main() {
  let subjects = sample_data::get_subjects();
  let timetables = generate_timetables(&subjects);

  loop {
    print!("Enter filter: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let filters = filter::parse_filters(&input);
    let filtered_timetables = filter::filter_timetables(timetables.clone(), filters);
    println!("Filtered timetables: {}", filtered_timetables.len());
    save_timetables(filtered_timetables);
  }
}
