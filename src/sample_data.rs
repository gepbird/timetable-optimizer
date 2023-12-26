use crate::data::{Course, CourseType, Enrollment, Occurence, Subject, WeekDay, Weeks};
use chrono::NaiveTime;

fn time(hours: u32, minutes: u32) -> NaiveTime {
  NaiveTime::from_hms_opt(hours, minutes, 0).unwrap()
}

pub fn get_subjects() -> Vec<Subject<'static>> {
  vec![
    Subject {
      name: "Electronics Basic",
      code: "NKXEAIEBNF",
      credits: 5,
      courses: vec![
        vec![Course {
          code: "EB_EA",
          course_type: CourseType::Lecture,
          enrollment: Enrollment {
            people_joined: 91,
            people_queue: 0,
            people_limit: 100,
          },
          location: "BA.F.05",
          teacher: "John Doe",
          language: "English",
          occurence: Occurence {
            weeks: Weeks::Every,
            week_day: WeekDay::Wednesday,
            start_time: time(14, 25),
            end_time: time(16, 5),
          },
        }],
        vec![
          Course {
            code: "EB_LA_01",
            course_type: CourseType::Laboratory,
            enrollment: Enrollment {
              people_joined: 22,
              people_queue: 0,
              people_limit: 24,
            },
            location: "BA.2.18",
            teacher: "John Doe",
            language: "English",
            occurence: Occurence {
              weeks: Weeks::Odd,
              week_day: WeekDay::Wednesday,
              start_time: time(16, 15),
              end_time: time(17, 50),
            },
          },
          Course {
            code: "EB_LA_02",
            course_type: CourseType::Laboratory,
            enrollment: Enrollment {
              people_joined: 24,
              people_queue: 0,
              people_limit: 24,
            },
            location: "BA.2.18",
            teacher: "John Doe",
            language: "English",
            occurence: Occurence {
              weeks: Weeks::Even,
              week_day: WeekDay::Wednesday,
              start_time: time(16, 15),
              end_time: time(17, 50),
            },
          },
          Course {
            code: "EB_LA_03",
            course_type: CourseType::Laboratory,
            enrollment: Enrollment {
              people_joined: 23,
              people_queue: 0,
              people_limit: 24,
            },
            location: "BA.2.18",
            teacher: "John Doe",
            language: "English",
            occurence: Occurence {
              weeks: Weeks::Odd,
              week_day: WeekDay::Wednesday,
              start_time: time(17, 55),
              end_time: time(19, 30),
            },
          },
          Course {
            code: "EB_LA_04",
            course_type: CourseType::Laboratory,
            enrollment: Enrollment {
              people_joined: 21,
              people_queue: 0,
              people_limit: 24,
            },
            location: "BA.2.18",
            teacher: "John Doe",
            language: "English",
            occurence: Occurence {
              weeks: Weeks::Even,
              week_day: WeekDay::Wednesday,
              start_time: time(17, 55),
              end_time: time(19, 30),
            },
          },
        ],
      ],
    },
    Subject {
      name: "Basic Mathematics",
      code: "NMXMA1EBNF",
      credits: 6,
      courses: vec![
        vec![Course {
          code: "BMaths_EA",
          course_type: CourseType::Lecture,
          enrollment: Enrollment {
            people_joined: 84,
            people_queue: 0,
            people_limit: 93,
          },
          location: "BA.F.08",
          teacher: "John Doe",
          language: "English",
          occurence: Occurence {
            weeks: Weeks::Every,
            week_day: WeekDay::Monday,
            start_time: time(8, 55),
            end_time: time(10, 35),
          },
        }],
        vec![
          Course {
            code: "BMaths_Gy_01",
            course_type: CourseType::Practice,
            enrollment: Enrollment {
              people_joined: 31,
              people_queue: 0,
              people_limit: 33,
            },
            location: "BA.F.03",
            teacher: "John Doe",
            language: "English",
            occurence: Occurence {
              weeks: Weeks::Every,
              week_day: WeekDay::Monday,
              start_time: time(10, 45),
              end_time: time(12, 25),
            },
          },
          Course {
            code: "BMaths_Gy_02",
            course_type: CourseType::Practice,
            enrollment: Enrollment {
              people_joined: 29,
              people_queue: 0,
              people_limit: 31,
            },
            location: "BA.F.03",
            teacher: "John Doe",
            language: "English",
            occurence: Occurence {
              weeks: Weeks::Every,
              week_day: WeekDay::Thursday,
              start_time: time(8, 0),
              end_time: time(9, 45),
            },
          },
          Course {
            code: "BMaths_Gy_03",
            course_type: CourseType::Practice,
            enrollment: Enrollment {
              people_joined: 24,
              people_queue: 0,
              people_limit: 30,
            },
            location: "BA.F.06",
            teacher: "John Doe",
            language: "English",
            occurence: Occurence {
              weeks: Weeks::Every,
              week_day: WeekDay::Wednesday,
              start_time: time(10, 45),
              end_time: time(12, 25),
            },
          },
        ],
      ],
    },
    Subject {
      name: "Learning Methodology",
      code: "NBXTM1EBNF",
      credits: 6,
      courses: vec![
        vec![Course {
          code: "LMet_EA",
          course_type: CourseType::Lecture,
          enrollment: Enrollment {
            people_joined: 81,
            people_queue: 0,
            people_limit: 89,
          },
          location: "BA.F.05",
          teacher: "John Doe",
          language: "English",
          occurence: Occurence {
            weeks: Weeks::Every,
            week_day: WeekDay::Friday,
            start_time: time(10, 45),
            end_time: time(12, 25),
          },
        }],
        vec![Course {
          code: "LMet_Gy",
          course_type: CourseType::Practice,
          enrollment: Enrollment {
            people_joined: 81,
            people_queue: 0,
            people_limit: 89,
          },
          location: "BA.F.05",
          teacher: "John Doe",
          language: "English",
          occurence: Occurence {
            weeks: Weeks::Every,
            week_day: WeekDay::Friday,
            start_time: time(10, 45),
            end_time: time(12, 25),
          },
        }],
      ],
    },
    Subject {
      name: "Mentoring",
      code: "NDIPT1EBNF",
      credits: 0,
      courses: vec![vec![Course {
        code: "Mentoring",
        course_type: CourseType::Practice,
        enrollment: Enrollment {
          people_joined: 82,
          people_queue: 0,
          people_limit: 90,
        },
        location: "BA.F.05",
        teacher: "John Doe",
        language: "English",
        occurence: Occurence {
          weeks: Weeks::Every,
          week_day: WeekDay::Friday,
          start_time: time(9, 50),
          end_time: time(10, 35),
        },
      }]],
    },
    Subject {
      name: "Problemsolving using programming",
      code: "NSXPP1EBNF",
      credits: 6,
      courses: vec![
        vec![Course {
          code: "PMP_EA_ENG",
          course_type: CourseType::Lecture,
          enrollment: Enrollment {
            people_joined: 82,
            people_queue: 0,
            people_limit: 91,
          },
          location: "BA.1.32.Audmax",
          teacher: "John Doe",
          language: "English",
          occurence: Occurence {
            weeks: Weeks::Every,
            week_day: WeekDay::Tuesday,
            start_time: time(14, 25),
            end_time: time(15, 10),
          },
        }],
        vec![
          Course {
            code: "PMP_LA_01_ENG",
            course_type: CourseType::Laboratory,
            enrollment: Enrollment {
              people_joined: 21,
              people_queue: 0,
              people_limit: 24,
            },
            location: "BC.2.201",
            teacher: "John Doe",
            language: "English",
            occurence: Occurence {
              weeks: Weeks::Every,
              week_day: WeekDay::Wednesday,
              start_time: time(11, 40),
              end_time: time(14, 15),
            },
          },
          Course {
            code: "PMP_LA_02_ENG",
            course_type: CourseType::Laboratory,
            enrollment: Enrollment {
              people_joined: 17,
              people_queue: 0,
              people_limit: 24,
            },
            location: "BA.2.15",
            teacher: "John Doe",
            language: "English",
            occurence: Occurence {
              weeks: Weeks::Every,
              week_day: WeekDay::Thursday,
              start_time: time(11, 40),
              end_time: time(14, 15),
            },
          },
          Course {
            code: "PMP_LA_03_ENG",
            course_type: CourseType::Laboratory,
            enrollment: Enrollment {
              people_joined: 24,
              people_queue: 0,
              people_limit: 24,
            },
            location: "BA.1.14",
            teacher: "John Doe",
            language: "English",
            occurence: Occurence {
              weeks: Weeks::Every,
              week_day: WeekDay::Monday,
              start_time: time(13, 30),
              end_time: time(16, 05),
            },
          },
          Course {
            code: "PMP_LA_04_ENG",
            course_type: CourseType::Laboratory,
            enrollment: Enrollment {
              people_joined: 20,
              people_queue: 0,
              people_limit: 24,
            },
            location: "BA.1.14",
            teacher: "John Doe",
            language: "English",
            occurence: Occurence {
              weeks: Weeks::Every,
              week_day: WeekDay::Thursday,
              start_time: time(9, 50),
              end_time: time(12, 25),
            },
          },
        ],
      ],
    },
    Subject {
      name: "Mathematical Foundations of Informatics",
      code: "NMXIMAEBNF",
      credits: 6,
      courses: vec![
        vec![Course {
          code: "MFI_EA",
          course_type: CourseType::Lecture,
          enrollment: Enrollment {
            people_joined: 86,
            people_queue: 0,
            people_limit: 94,
          },
          location: "BA.F.05",
          teacher: "John Doe",
          language: "English",
          occurence: Occurence {
            weeks: Weeks::Every,
            week_day: WeekDay::Tuesday,
            start_time: time(8, 00),
            end_time: time(9, 40),
          },
        }],
        vec![
          Course {
            code: "MFI_Gy_01",
            course_type: CourseType::Practice,
            enrollment: Enrollment {
              people_joined: 31,
              people_queue: 0,
              people_limit: 33,
            },
            location: "BA.F.03",
            teacher: "John Doe",
            language: "English",
            occurence: Occurence {
              weeks: Weeks::Every,
              week_day: WeekDay::Tuesday,
              start_time: time(9, 50),
              end_time: time(12, 25),
            },
          },
          Course {
            code: "MFI_Gy_02",
            course_type: CourseType::Practice,
            enrollment: Enrollment {
              people_joined: 29,
              people_queue: 0,
              people_limit: 32,
            },
            location: "BC.4.402",
            teacher: "John Doe",
            language: "English",
            occurence: Occurence {
              weeks: Weeks::Every,
              week_day: WeekDay::Wednesday,
              start_time: time(11, 40),
              end_time: time(14, 15),
            },
          },
          Course {
            code: "MFI_Gy_03",
            course_type: CourseType::Practice,
            enrollment: Enrollment {
              people_joined: 26,
              people_queue: 0,
              people_limit: 33,
            },
            location: "BC.4.402",
            teacher: "John Doe",
            language: "English",
            occurence: Occurence {
              weeks: Weeks::Every,
              week_day: WeekDay::Thursday,
              start_time: time(13, 30),
              end_time: time(16, 5),
            },
          },
        ],
      ],
    },
  ]
}
