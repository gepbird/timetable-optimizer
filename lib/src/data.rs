use std::fmt::{self, Display, Formatter};

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use chrono::{NaiveTime, Weekday};
use serde::{Deserialize, Serialize};
use sha2::{
  digest::{generic_array::GenericArray, typenum::U32},
  Digest, Sha256,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
  pub name: String,
  pub courses: Vec<OneOfCourse>,
}

pub type OneOfCourse = Vec<Course>;

#[derive(Debug, Clone, Serialize)]
pub struct Timetable<'a> {
  pub id: u32,
  pub courses: Vec<&'a Course>,
  hash: Option<String>,
}

impl<'a> Timetable<'a> {
  pub fn new(id: u32, courses: Vec<&'a Course>) -> Timetable<'a> {
    Timetable {
      id,
      courses,
      hash: None,
    }
  }

  pub fn hash(&mut self) -> &str {
    if self.hash.is_none() {
      self.update_hash();
    }

    self.hash.as_ref().unwrap()
  }

  fn update_hash(&mut self) {
    let mut hasher = Sha256::new();
    for course in &self.courses {
      hasher.update(course.hash);
    }

    let hash = URL_SAFE.encode(hasher.finalize());
    self.hash = Some(hash);
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
  pub subject_name: String,
  pub code: String,
  pub course_type: CourseType,
  #[serde(skip)]
  pub enrollment: Enrollment,
  pub location: String,
  pub teacher: String,
  pub language: String,
  pub site: String,
  pub comment: String,
  pub description: String,
  pub occurrence: Occurrence,
  #[serde(skip)]
  hash: GenericArray<u8, U32>,
  #[serde(skip)]
  pub is_deleted: bool,
  #[serde(skip)]
  pub is_hidden_by_user: bool,
}

impl Course {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    subject_name: String,
    code: String,
    course_type: CourseType,
    enrollment: Enrollment,
    location: String,
    teacher: String,
    language: String,
    site: String,
    comment: String,
    description: String,
    occurrence: Occurrence,
  ) -> Course {
    let mut course = Course {
      subject_name,
      code,
      course_type,
      enrollment,
      location,
      teacher,
      language,
      site,
      comment,
      description,
      occurrence,
      hash: GenericArray::default(),
      is_deleted: false,
      is_hidden_by_user: false,
    };

    course.update_hash();

    course
  }

  pub fn update_hash(&mut self) {
    let serialized = serde_json::to_string(&self).unwrap();
    self.hash = Sha256::digest(serialized.as_bytes());
  }

  pub fn is_ignored(&self) -> bool {
    self.is_deleted || self.is_hidden_by_user
  }
}

#[derive(
  Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display,
)]
pub enum CourseType {
  Lecture,
  Laboratory,
  Practice,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Enrollment {
  pub people_joined: u32,
  pub people_queue: u32,
  pub people_limit: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Occurrence {
  pub weeks: Option<Weeks>,
  pub weekday: Weekday,
  pub start_time: NaiveTime,
  pub end_time: NaiveTime,
}

impl Display for Occurrence {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{} {}-{}", self.weekday, self.start_time, self.end_time)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum Weeks {
  Every,
  Even,
  Odd,
}
