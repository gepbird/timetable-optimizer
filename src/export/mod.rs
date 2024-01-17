use std::fs;

use indicatif::ProgressBar;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::data::Timetable;

pub mod course_code;
pub mod image;
pub mod json;

pub fn save_timetables_parallel(timetables: Vec<Timetable>) {
  let out_dir = "out";
  fs::create_dir_all(out_dir).unwrap();

  let full_dir = format!("{out_dir}/full");
  let codes_dir = format!("{out_dir}/course-codes");
  let images_dir = format!("{out_dir}/images");
  for subdirectory in vec![&full_dir, &codes_dir, &images_dir] {
    fs::create_dir_all(&subdirectory).ok();
    for entry in fs::read_dir(subdirectory).unwrap() {
      fs::remove_file(entry.unwrap().path()).unwrap();
    }
  }

  let progress_bar = ProgressBar::new(timetables.len() as u64);

  timetables
    .par_iter()
    .enumerate()
    .for_each(|(index, timetable)| {
      let name = format!("timetable_{index:04}");
      json::save_timetable_json(timetable, format!("{full_dir}/{name}.json"));
      course_code::save_course_codes(timetable, format!("{codes_dir}/{name}.txt"));
      image::save_timetable_image(timetable, format!("{images_dir}/{name}.bmp"));
      progress_bar.inc(1);
    });

  progress_bar.finish();
}
