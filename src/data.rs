use chrono::NaiveTime;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Subject<'a> {
  pub name: &'a str,
  pub code: &'a str,
  pub credits: u32,
  pub courses: Vec<OneOfCourse<'a>>,
}

pub type OneOfCourse<'a> = Vec<Course<'a>>;
pub type Timetable<'a> = Vec<&'a Course<'a>>;

#[derive(Debug, Clone, Serialize)]
pub struct Course<'a> {
  pub code: &'a str,
  pub course_type: CourseType,
  pub enrollment: Enrollment,
  pub location: &'a str,
  pub teacher: &'a str,
  pub language: &'a str,
  pub occurence: Occurence,
}

#[derive(Debug, Clone, Serialize)]
pub enum CourseType {
  Lecture,
  Laboratory,
  Practice,
}

#[derive(Debug, Clone, Serialize)]
pub struct Enrollment {
  pub people_joined: u32,
  pub people_queue: u32,
  pub people_limit: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct Occurence {
  pub weeks: Weeks,
  pub week_day: WeekDay,
  pub start_time: NaiveTime,
  pub end_time: NaiveTime,
}

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub enum Weeks {
  Every,
  Even,
  Odd,
}

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub enum WeekDay {
  Monday = 0,
  Tuesday = 1,
  Wednesday = 2,
  Thursday = 3,
  Friday = 4,
  Saturday = 5,
  Sunday = 6,
}
