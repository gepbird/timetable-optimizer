use chrono::{NaiveTime, Weekday};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
  pub name: String,
  pub code: String,
  pub credits: u32,
  pub courses: Vec<OneOfCourse>,
}

pub type OneOfCourse = Vec<Course>;

#[derive(Debug, Clone, Serialize)]
pub struct Timetable<'a> {
  pub id: u32,
  pub courses: Vec<&'a Course>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
  pub subject_name: String,
  pub code: String,
  pub course_type: CourseType,
  pub enrollment: Enrollment,
  pub location: String,
  pub teacher: String,
  pub language: String,
  pub site: String,
  pub comment: String,
  pub description: String,
  pub occurrence: Occurrence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CourseType {
  #[serde(alias = "Elmélet")]
  Lecture,
  #[serde(alias = "Labor")]
  Laboratory,
  #[serde(alias = "Gyakorlat")]
  Practice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enrollment {
  pub people_joined: u32,
  pub people_queue: u32,
  pub people_limit: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Occurrence {
  pub weeks: Weeks,
  pub weekday: Weekday,
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
