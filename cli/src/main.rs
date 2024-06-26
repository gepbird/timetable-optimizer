use std::env;

use itertools::Itertools;
use permutator::CartesianProduct;
use timetable_optimizer_lib::data::{Course, Subject, Timetable};

mod export;
mod filter;
mod sample_data;
mod setup;

pub fn generate_timetables<'a>(subjects: &'a [Subject]) -> Vec<Timetable<'a>> {
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
    .map(|(i, cp)| {
      Timetable::new(
        i as u32,
        cp.into_iter()
          .copied()
          .sorted_by_key(|course| course.occurrence.start_time)
          .sorted_by_key(|course| course.occurrence.weekday as u8)
          .collect::<Vec<&'a Course>>(),
      )
    })
    // filter out overlapping courses
    .filter(|timetable| {
      timetable
        .courses
        .iter()
        .chunk_by(|course| course.occurrence.weekday)
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
    setup::setup()
  } else {
    sample_data::get_subjects()
  };

  let timetables = generate_timetables(&subjects);

  loop {
    let filters = filter::prompt_filters();
    // TODO: due to cloning, mutating won't persist (saving cached hash)
    let mut filtered_timetables = filter::filter_timetables(timetables.clone(), filters);
    println!("Filtered timetables: {}", filtered_timetables.len());
    export::save_filtered(&mut filtered_timetables);
  }
}
