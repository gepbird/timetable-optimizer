use crate::data::{OneOfCourse, Subject};

fn count_one_of_courses(courses: &OneOfCourse) -> u64 {
  courses.iter().filter(|&c| !c.is_ignored()).count() as u64
}

pub fn count_all_courses(subjects: &[Subject]) -> u64 {
  subjects
    .iter()
    .map(|subject| {
      subject
        .courses
        .iter()
        .map(count_one_of_courses)
        .filter(|&count| count > 0)
        .sum::<u64>()
    })
    .sum()
}

pub fn count_course_per_timetable(subjects: &[Subject]) -> u64 {
  subjects
    .iter()
    .map(|subject| {
      subject
        .courses
        .iter()
        .filter(|one_of_courses| count_one_of_courses(one_of_courses) > 0)
        .count() as u64
    })
    .sum()
}

pub fn count_all_timetables(subjects: &[Subject]) -> u64 {
  subjects
    .iter()
    .map(|subject| {
      subject
        .courses
        .iter()
        .map(count_one_of_courses)
        .filter(|&count| count > 0)
        .product::<u64>()
    })
    .product()
}
