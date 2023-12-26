use permutator::CartesianProduct;
use std::fs;
use std::fs::File;
use std::io::Write;

use data::Course;
use data::Subject;
use data::Timetable;

mod data;
mod sample_data;

fn save_timetables(timetables: Vec<Vec<&Course>>) {
  fs::remove_dir_all("out").ok();
  fs::create_dir_all("out").unwrap();

  for (index, timetable) in timetables.iter().enumerate() {
    let file_name = format!("out/timetable_{:04}.json", index);
    let serialized_timetable = serde_json::to_string_pretty(timetable).unwrap();

    File::create(file_name)
      .unwrap()
      .write_all(serialized_timetable.as_bytes())
      .unwrap();
  }
}

pub fn generate_timetables<'a>(subjects: &'a Vec<Subject<'a>>) -> Vec<Timetable> {
  let one_of_courses: Vec<Vec<&'a Course<'a>>> = subjects
    .iter()
    .flat_map(|subject| &subject.courses)
    .map(|one_of_course| one_of_course.iter().collect::<Vec<&'a Course<'a>>>())
    .collect();

  let timetables: Vec<Vec<&'a Course<'a>>> = one_of_courses
    .iter()
    .map(|x| x.as_slice())
    .collect::<Vec<&[&'a Course<'a>]>>()
    .cart_prod()
    .map(|cp| {
      cp.iter()
        .map(|&&course| course)
        .collect::<Vec<&'a Course<'a>>>()
    })
    .collect();

  timetables
}

fn main() {
  let subjects = sample_data::get_subjects();
  let timetables = generate_timetables(&subjects);
  save_timetables(timetables);
}
