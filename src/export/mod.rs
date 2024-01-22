use std::os::unix::fs as unix_fs;
use std::path::PathBuf;
use std::{env, fs};

use indicatif::ProgressBar;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::data::Timetable;

pub mod course_code;
pub mod image;
pub mod json;

pub fn save_filtered(timetables: &mut [Timetable]) {
  clean_legacy();

  let img_ext = if cfg!(debug_assertions) { "bmp" } else { "png" };
  let current_dir = env::current_dir().unwrap();
  let store = current_dir.join("out/store");
  let filtered = current_dir.join("out/filtered");

  make_cleaned_dirs(&[
    &filtered.join("json"),
    &filtered.join("course-code"),
    &filtered.join("image"),
  ]);
  fs::create_dir_all(&store).ok();

  let progress_bar = ProgressBar::new(timetables.len() as u64);

  timetables.par_iter_mut().for_each(|timetable| {
    let id = timetable.id;
    let hash = timetable.hash();
    let store_item = store.join(hash);

    let json_store = store_item.join(format!("timetable.json"));
    let course_codes_store = store_item.join(format!("timetable.txt"));
    let image_store = store_item.join(format!("timetable.{img_ext}"));
    let json_filtered = filtered.join(format!("json/timetable_{id}.json"));
    let course_code_filtered = filtered.join(format!("course-code/timetable_{id}.txt"));
    let image_filtered = filtered.join(format!("image/timetable_{id}.{img_ext}"));

    unix_fs::symlink(&json_store, &json_filtered).unwrap();
    unix_fs::symlink(&course_codes_store, &course_code_filtered).unwrap();
    unix_fs::symlink(&image_store, &image_filtered).unwrap();

    if !store_item.exists() {
      fs::create_dir(&store_item).unwrap();
      json::save_timetable_json(timetable, store_item.join(json_store));
      course_code::save_course_codes(timetable, store_item.join(course_codes_store));
      image::save_timetable_image(timetable, store_item.join(image_store));
    }

    progress_bar.inc(1);
  });

  progress_bar.finish();
}

fn make_cleaned_dirs(dirs: &[&PathBuf]) {
  for subdir in dirs {
    fs::create_dir_all(&subdir).ok();
    for entry in fs::read_dir(subdir).unwrap() {
      fs::remove_file(entry.unwrap().path()).unwrap();
    }
  }
}

fn clean_legacy() {
  let current_dir = env::current_dir().unwrap();
  let all_dir = current_dir.join("out/all");
  fs::remove_dir_all(all_dir).ok();
}
