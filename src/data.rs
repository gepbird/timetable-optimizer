use chrono::{NaiveTime, Weekday};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
  pub name: String,
  pub code: String,
  pub credits: u32,
  pub courses: Vec<OneOfCourse>,
}

pub type OneOfCourse = Vec<Course>;
pub type Timetable<'a> = Vec<&'a Course>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
  pub code: String,
  pub course_type: CourseType,
  pub enrollment: Enrollment,
  pub location: String,
  pub teacher: String,
  pub language: String,
  pub occurence: Occurence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CourseType {
  Lecture,
  Laboratory,
  Practice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enrollment {
  pub people_joined: u32,
  pub people_queue: u32,
  pub people_limit: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Occurence {
  pub weeks: Weeks,
  pub week_day: Weekday,
  pub start_time: NaiveTime,
  pub end_time: NaiveTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum Weeks {
  Every,
  Even,
  Odd,
}
