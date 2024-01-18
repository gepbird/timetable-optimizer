use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use cached::proc_macro::cached;
use chrono::{Duration, NaiveTime, Weekday};
use image::{ImageBuffer, Rgba};
use imageproc::{drawing, rect::Rect};
use rusttype::{Font, Scale};

use crate::data::Timetable;

type Color = Rgba<u8>;
type Image = ImageBuffer<Color, Vec<u8>>;

const HEADER_HEIGHT: u32 = 50;
const TIMES_WIDTH: u32 = 100;
const DAY_WIDTH: u32 = 150;
const MINUTE_HEIGHT: f32 = 1f32;
const DAY_COUNT: u32 = 6; // from monday to saturday
const PADDING: i32 = 5;
const CANVAS_WIDTH: u32 = TIMES_WIDTH + DAY_WIDTH * DAY_COUNT;
const VERTICAL_LINE_THICKNESS: u32 = 4;

const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);
const GRAY: Rgba<u8> = Rgba([128, 128, 128, 255]);
const DARK_GRAY: Rgba<u8> = Rgba([64, 64, 64, 255]);
const BLACK: Rgba<u8> = Rgba([0, 0, 0, 255]);

pub fn save_timetable_image(timetable: &Timetable, file_path: String) {
  let day_start = NaiveTime::from_hms_opt(8, 0, 0).unwrap();
  let day_end = NaiveTime::from_hms_opt(20, 0, 0).unwrap();
  let day_length = day_end - day_start;
  let day_height = day_length.num_minutes() as f32 * MINUTE_HEIGHT;
  let hours = day_length.num_hours();
  let canvas_height = HEADER_HEIGHT + day_height as u32;

  let mut img = draw_timetable_base_cached(canvas_height, hours, day_start);
  draw_courses(timetable, day_start, &mut img);

  img.save(file_path.as_str()).unwrap();
}

#[cached]
fn draw_timetable_base_cached(canvas_height: u32, hours: i64, day_start: NaiveTime) -> Image {
  let mut img = Image::new(CANVAS_WIDTH, canvas_height);

  clear(&mut img, canvas_height);
  draw_hours_with_lines(&mut img, hours, day_start);
  draw_half_hour_lines(&mut img, hours);
  draw_days_with_lines(&mut img, canvas_height);

  img
}

fn draw_courses(
  timetable: &Timetable,
  day_start: NaiveTime,
  img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
) {
  for course in &timetable.courses {
    let occ = &course.occurrence;
    let weekday = course.occurrence.weekday.number_from_monday() - 1;
    let duration = course.occurrence.end_time - course.occurrence.start_time;

    let x = TIMES_WIDTH + weekday * DAY_WIDTH + VERTICAL_LINE_THICKNESS / 2;
    let width = DAY_WIDTH - VERTICAL_LINE_THICKNESS;
    let start_minutes = (occ.start_time - day_start).num_minutes();
    let y = HEADER_HEIGHT as f32 + start_minutes as f32 * MINUTE_HEIGHT;
    let height = duration.num_minutes() as f32 * MINUTE_HEIGHT;
    let rect = Rect::at(x as i32, y as i32).of_size(width, height as u32);

    let background = color_hash(&course.code);
    let average_color =
      background.0.iter().map(|&x| x as u16).sum::<u16>() / background.0.len() as u16;
    let foreground = match average_color as u8 {
      0..=127 => WHITE,
      128..=255 => BLACK,
    };

    drawing::draw_filled_rect_mut(img, rect, background);
    drawing::draw_text_mut(
      img,
      foreground,
      x as i32 + PADDING,
      y as i32 + PADDING,
      Scale { x: 16.0, y: 16.0 },
      &get_font(),
      &course.code,
    );
  }
}

fn draw_days_with_lines(
  img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
  canvas_height: u32,
) {
  for day_seperator in 0..DAY_COUNT {
    let start_x = day_seperator * DAY_WIDTH + TIMES_WIDTH;
    draw_thick_line(
      img,
      (start_x, 0),
      (start_x, canvas_height),
      VERTICAL_LINE_THICKNESS,
      GRAY,
    );

    let day_name = Weekday::try_from(day_seperator as u8).unwrap().to_string();
    drawing::draw_text_mut(
      img,
      WHITE,
      start_x as i32 + 10,
      10 as i32,
      Scale { x: 30.0, y: 30.0 },
      &get_font(),
      &day_name,
    );
  }
}

fn draw_half_hour_lines(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, hours: i64) {
  for hour in 0..hours * 2 {
    let y = HEADER_HEIGHT as f32 + hour as f32 * 30f32 * MINUTE_HEIGHT;
    draw_thick_line(img, (0, y as u32), (CANVAS_WIDTH, y as u32), 2, DARK_GRAY);
  }
}

fn draw_hours_with_lines(
  img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
  hours: i64,
  day_start: NaiveTime,
) {
  for hour in 0..hours {
    let y = HEADER_HEIGHT as f32 + hour as f32 * 60f32 * MINUTE_HEIGHT;
    let time = day_start + Duration::hours(hour);
    draw_thick_line(img, (0, y as u32), (CANVAS_WIDTH, y as u32), 4, GRAY);
    drawing::draw_text_mut(
      img,
      WHITE,
      PADDING,
      y as i32 + PADDING,
      Scale { x: 16.0, y: 16.0 },
      &get_font(),
      time.format("%H:%M").to_string().as_str(),
    );
  }
}

fn clear(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, canvas_height: u32) {
  drawing::draw_filled_rect_mut(
    img,
    Rect::at(0, 0).of_size(CANVAS_WIDTH, canvas_height),
    BLACK,
  );
}

fn draw_thick_line(
  img: &mut Image,
  start: (u32, u32),
  end: (u32, u32),
  thickness: u32,
  color: Color,
) {
  let half_thick = thickness / 2;
  let is_horizontal = start.1 == end.1;
  let rect = if is_horizontal {
    Rect::at(start.0 as i32, (start.1 - half_thick) as i32).of_size(end.0 - start.0, thickness)
  } else {
    Rect::at((start.0 - half_thick) as i32, start.1 as i32).of_size(thickness, end.1 - start.1)
  };
  drawing::draw_filled_rect_mut(img, rect, color);
}

fn get_font<'a>() -> Font<'a> {
  let font_data: &[u8] = include_bytes!("../../data/Helvetica.ttf");
  let font = Font::try_from_bytes(font_data).unwrap();
  font
}

fn color_hash(course_code: &str) -> Rgba<u8> {
  let mut hasher = DefaultHasher::new();
  course_code.hash(&mut hasher);
  let hash = hasher.finish();

  let red = (hash & 0xFF) as u8;
  let green = ((hash >> 8) & 0xFF) as u8;
  let blue = ((hash >> 16) & 0xFF) as u8;
  Rgba([red, green, blue, 255])
}
