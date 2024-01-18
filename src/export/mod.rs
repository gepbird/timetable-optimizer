use std::{env, fs};

use indicatif::ProgressBar;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::data::Timetable;

pub mod course_code;
pub mod image;
pub mod json;

pub fn save_timetables_parallel(timetables: &[Timetable]) {
  let current_dir = env::current_dir().unwrap();
  let out_dir = current_dir.join("out");

  let full_dir = out_dir.join("full");
  let codes_dir = out_dir.join("course-codes");
  let images_dir = out_dir.join("images");
  for subdirectory in &[&full_dir, &codes_dir, &images_dir] {
    fs::create_dir_all(&subdirectory).ok();
    for entry in fs::read_dir(subdirectory).unwrap() {
      fs::remove_file(entry.unwrap().path()).unwrap();
    }
  }

  let progress_bar = ProgressBar::new(timetables.len() as u64);

  timetables.par_iter().for_each(|timetable| {
    let id = timetable.id;
    let name = format!("timetable_{id:04}");
    let image_extension = if cfg!(debug_assertions) { "bmp" } else { "png" };

    json::save_timetable_json(timetable, full_dir.join(format!("{name}.json")));
    course_code::save_course_codes(timetable, codes_dir.join(format!("{name}.txt")));
    image::save_timetable_image(
      timetable,
      images_dir.join(format!("{name}.{image_extension}")),
    );

    progress_bar.inc(1);
  });

  progress_bar.finish();
}
