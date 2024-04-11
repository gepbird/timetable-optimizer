use crate::data::Subject;

pub fn count_all_courses(subjects: &[Subject]) -> u64 {
  subjects
    .iter()
    .map(|subject| {
      subject
        .courses
        .iter()
        .map(|one_of_courses| one_of_courses.len() as u64)
        .sum::<u64>()
    })
    .sum()
}

pub fn count_course_per_timetable(subjects: &[Subject]) -> u64 {
  subjects
    .iter()
    .map(|subject| subject.courses.len() as u64)
    .sum()
}

pub fn count_all_timetables(subjects: &[Subject]) -> u64 {
  subjects
    .iter()
    .map(|subject| {
      subject
        .courses
        .iter()
        .map(|one_of_courses| one_of_courses.len() as u64)
        .product::<u64>()
    })
    .product()
}
