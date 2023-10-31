use chrono::NaiveTime;

pub struct Subject<'a> {
  pub name: &'a str,
  pub code: &'a str,
  pub credits: u32,
  pub courses: Vec<OneOfCourse>,
}

type OneOfCourse = Vec<Course<'static>>;
pub struct Course<'a> {
  pub code: &'a str,
  pub course_type: CourseType,
  pub people_joined: u32,
  pub people_queue: u32,
  pub people_limit: u32,
  pub start_time: WeekTime,
  pub end_time: WeekTime,
  pub location: &'a str,
  pub teacher: &'a str,
  pub language : &'a str,
  pub occurence: Occurence,
}

pub enum CourseType {
  Lecture,
  Laboratory,
  Practice,
}

pub struct WeekTime {
  pub week_day: WeekDay,
  pub time: NaiveTime,
}

pub enum WeekDay {
  Monday = 0,
  Tuesday = 1,
  Wednesday = 2,
  Thursday = 3,
  Friday = 4,
  Saturday = 5,
  Sunday = 6,
}

pub enum Occurence {
  EveryWeeks,
  EvenWeeks,
  OddWeeks,
}
