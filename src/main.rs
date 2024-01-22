use std::env;

use data::{Course, Subject, Timetable};
use itertools::Itertools;
use permutator::CartesianProduct;

mod data;
mod excel;
mod export;
mod filter;
mod sample_data;
mod setup;

pub fn generate_timetables<'a>(subjects: &'a Vec<Subject>) -> Vec<Timetable<'a>> {
  let one_of_courses: Vec<Vec<&'a Course>> = subjects
    .iter()
    .flat_map(|subject| &subject.courses)
    .map(|one_of_course| one_of_course.iter().collect::<Vec<&'a Course>>())
    .collect();

  let timetables: Vec<Timetable> = one_of_courses
    .iter()
    .map(|x| x.as_slice())
    .collect::<Vec<&[&'a Course]>>()
    .cart_prod()
    .enumerate()
    .map(|(i, cp)| Timetable {
      id: i as u32,
      courses: cp
        .into_iter()
        .map(|&course| course)
        .sorted_by_key(|course| course.occurrence.start_time)
        .sorted_by_key(|course| course.occurrence.weekday as u8)
        .collect::<Vec<&'a Course>>(),
    })
    // filter out overlapping courses
    .filter(|timetable| {
      timetable
        .courses
        .iter()
        .group_by(|course| course.occurrence.weekday)
        .into_iter()
        .all(|(_, courses)| {
          courses
            .tuple_windows()
            .all(|(current, next)| next.occurrence.start_time >= current.occurrence.end_time)
        })
    })
    .collect();

  timetables
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let subjects: Vec<Subject> = if args.contains(&"--setup".to_string()) {
    setup::import_subjects()
  } else {
    sample_data::get_subjects()
  };

  let timetables = generate_timetables(&subjects);
  export::save_timetables_parallel(&timetables);

  loop {
    let filters = filter::prompt_filters();
    let filtered_timetables = filter::filter_timetables(timetables.clone(), filters);
    println!("Filtered timetables: {}", filtered_timetables.len());
    export::symlink_filtered_timetables(&filtered_timetables);
  }
}
