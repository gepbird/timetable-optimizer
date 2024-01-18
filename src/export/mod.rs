use std::os::unix::fs as unix_fs;
use std::path::PathBuf;
use std::{env, fs};

use indicatif::ProgressBar;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::data::Timetable;

pub mod course_code;
pub mod image;
pub mod json;

pub fn save_timetables_parallel(timetables: &[Timetable]) {
  let current_dir = env::current_dir().unwrap();
  let all_dir = current_dir.join("out/all");

  let full_dir = all_dir.join("json");
  let codes_dir = all_dir.join("course-code");
  let images_dir = all_dir.join("image");

  make_cleaned_dirs(&[&full_dir, &codes_dir, &images_dir]);

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

pub fn symlink_filtered_timetables(timetables: &[Timetable]) {
  let current_dir = env::current_dir().unwrap();
  let all_dir = current_dir.join("out/all");
  let filtered_dir = current_dir.join("out/filtered");

  make_cleaned_dirs(&[
    &filtered_dir.join("json"),
    &filtered_dir.join("course-code"),
    &filtered_dir.join("image"),
  ]);

  for timetable in timetables {
    let symlink = |export_type: &str, file_name: &str| {
      unix_fs::symlink(
        all_dir.join(export_type).join(file_name),
        filtered_dir.join(export_type).join(file_name),
      )
      .unwrap();
    };

    let id = timetable.id;
    let name = format!("timetable_{id:04}");
    let image_extension = if cfg!(debug_assertions) { "bmp" } else { "png" };

    symlink("json", &format!("{name}.json"));
    symlink("course-code", &format!("{name}.txt"));
    symlink("image", &format!("{name}.{image_extension}"));
  }
}

fn make_cleaned_dirs(dirs: &[&PathBuf]) {
  for subdir in dirs {
    fs::create_dir_all(&subdir).ok();
    for entry in fs::read_dir(subdir).unwrap() {
      fs::remove_file(entry.unwrap().path()).unwrap();
    }
  }
}
