use std::fs::File;
use std::io::Write;

use data::Subject;
use data::Timetable;

mod data;
mod sample_data;

fn generate_combinations(subjects: Vec<Subject>) -> Vec<Timetable> {
  let mut timetables = Vec::new();
  generate_combinations_recursive(&subjects, Vec::new(), &mut timetables, 0);
  timetables
}

fn generate_combinations_recursive<'a>(
  subjects: &Vec<Subject<'a>>,
  current_timetable: Timetable<'a>,
  timetables: &mut Vec<Timetable<'a>>,
  subject_index: usize,
) {
  if subject_index == subjects.len() {
    // If we reached the end of subjects, add the current timetable to the list
    timetables.push(current_timetable.clone());
    return;
  }

  for optional_courses in &subjects[subject_index].courses {
    for course in optional_courses {
      let mut new_timetable = current_timetable.clone();
      new_timetable.push(course.clone());

      generate_combinations_recursive(subjects, new_timetable, timetables, subject_index + 1);
    }
  }
}

fn write_timetables(timetables: Vec<Timetable>) {
  for (index, timetable) in timetables.iter().enumerate() {
    let file_name =format!("out/timetable_{:04}.json", index);
    let serialized_timetable = serde_json::to_string_pretty(timetable).unwrap();

    File::create(file_name)
      .unwrap()
      .write_all(serialized_timetable.as_bytes())
      .unwrap();
  }
}

fn main() {
  let subjects = sample_data::get_subjects();
  let timetables = generate_combinations(subjects);
  write_timetables(timetables);
}
