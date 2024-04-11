use rustyline::{history::DefaultHistory, Editor};

use timetable_optimizer_lib::data::Timetable;

pub mod exclude_teacher;
pub mod free_workdays;
pub mod max_end_time;
pub mod max_gap_between_courses;
pub mod min_start_time;
pub mod no_course_between;

pub trait Filter {
  fn filter(&self, timetable: &Timetable) -> bool;
}

fn parse_with_key<F, T>(
  spec: &str,
  key: &str,
  parse_fn: F,
) -> Option<Result<Box<dyn Filter>, String>>
where
  F: FnOnce(&str) -> Result<T, String>,
  T: Filter + 'static,
{
  // TODO: remove when fixed: https://github.com/rust-lang/rust-clippy/issues/12659
  #[allow(clippy::manual_map)]
  match spec.strip_prefix(&(key.to_string() + "=")) {
    Some(value) => Some(match parse_fn(value) {
      Ok(filter) => Ok(Box::new(filter)),
      Err(e) => Err(e),
    }),
    None => None,
  }
}

fn parse_filter(spec: &str) -> Result<Box<dyn Filter>, String> {
  let parsers = &[
    min_start_time::try_parse,
    max_end_time::try_parse,
    free_workdays::try_parse,
    max_gap_between_courses::try_parse,
    exclude_teacher::try_parse,
    no_course_between::try_parse,
  ];

  parsers
    .iter()
    .find_map(|parser| parser(spec))
    .ok_or_else(|| format!("Invalid filter specification: {spec}"))?
}

pub fn prompt_filters() -> Vec<Box<dyn Filter>> {
  let mut rl = Editor::<(), DefaultHistory>::new().unwrap();
  let hist_file = "out/history.txt";
  rl.load_history(hist_file).ok();
  let specs = rl.readline("Enter filter: ").unwrap();
  rl.add_history_entry(specs.as_str()).unwrap();
  rl.save_history(hist_file).unwrap();

  let filters_parsed = specs
    .trim()
    .split(' ')
    .filter(|spec| !spec.is_empty())
    .map(parse_filter)
    .collect();

  match filters_parsed {
    Ok(filters) => filters,
    Err(e) => {
      eprintln!("Error parsing filter: {e}");
      prompt_filters()
    }
  }
}

pub fn filter_timetables(
  timetables: Vec<Timetable>,
  filters: Vec<Box<dyn Filter>>,
) -> Vec<Timetable> {
  timetables
    .into_iter()
    .filter(|timetable| filters.iter().all(|filter| filter.filter(timetable)))
    .collect()
}
