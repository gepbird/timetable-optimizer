use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use chrono::{NaiveTime, Duration, Weekday};
use image::{Rgba, ImageBuffer};
use imageproc::{drawing, rect::Rect};
use rusttype::{Scale, Font};

use crate::data::Timetable;

type Color = Rgba<u8>;
type Image = ImageBuffer<Color, Vec<u8>>;

pub fn save_timetable_image(timetable: &Timetable, file_path: String) {
  let draw_time = std::time::Instant::now();

  let header_height = 50;
  let times_width = 100;
  let day_width = 150;
  let day_start = NaiveTime::from_hms_opt(8, 0, 0).unwrap();
  let day_end = NaiveTime::from_hms_opt(20, 0, 0).unwrap();
  let minute_height = 1f32;
  let day_length = day_end - day_start;
  let day_height = day_length.num_minutes() as f32 * minute_height;
  let day_count = 6; // from monday to saturday
  let padding = 5;

  let canvas_width = times_width + day_width * day_count;
  let canvas_height = header_height + day_height as u32;
  let mut img = Image::new(canvas_width, canvas_height);

  let font_data: &[u8] = include_bytes!("../../data/Helvetica.ttf");
  let font = Font::try_from_bytes(font_data).unwrap();

  let white = Rgba([255, 255, 255, 255]);
  let gray = Rgba([128, 128, 128, 255]);
  let dark_gray = Rgba([64, 64, 64, 255]);
  let black = Rgba([0, 0, 0, 255]);
  drawing::draw_filled_rect_mut(
    &mut img,
    Rect::at(0, 0).of_size(canvas_width, canvas_height),
    black,
  );

  let hours = day_length.num_hours();
  for hour in 0..hours {
    let y = header_height as f32 + hour as f32 * 60f32 * minute_height;
    let time = day_start + Duration::hours(hour);
    draw_thick_line(&mut img, (0, y as u32), (canvas_width, y as u32), 4, gray);
    drawing::draw_text_mut(
      &mut img,
      white,
      padding,
      y as i32 + padding,
      Scale { x: 16.0, y: 16.0 },
      &font,
      time.format("%H:%M").to_string().as_str(),
    );
  }

  for hour in 0..hours * 2 {
    let y = header_height as f32 + hour as f32 * 30f32 * minute_height;
    draw_thick_line(
      &mut img,
      (0, y as u32),
      (canvas_width, y as u32),
      2,
      dark_gray,
    );
  }

  for course in timetable {
    let occ = &course.occurrence;
    let weekday = course.occurrence.weekday.number_from_monday() - 1;
    let duration = course.occurrence.end_time - course.occurrence.start_time;

    let x = times_width + weekday * day_width;
    let width = day_width;
    let start_minutes = (occ.start_time - day_start).num_minutes();
    let y = header_height as f32 + start_minutes as f32 * minute_height;
    let height = duration.num_minutes() as f32 * minute_height;
    let rect = Rect::at(x as i32, y as i32).of_size(width, height as u32);

    let background = color_hash(&course.code);
    let average_color =
      background.0.iter().map(|&x| x as u16).sum::<u16>() / background.0.len() as u16;
    let foreground = match average_color as u8 {
      0..=127 => white,
      128..=255 => black,
    };

    drawing::draw_filled_rect_mut(&mut img, rect, background);
    drawing::draw_text_mut(
      &mut img,
      foreground,
      x as i32 + padding,
      y as i32 + padding,
      Scale { x: 16.0, y: 16.0 },
      &font,
      &course.code,
    );
  }

  for day_seperator in 0..day_count {
    let start_x = day_seperator * day_width + times_width;
    draw_thick_line(&mut img, (start_x, 0), (start_x, canvas_height), 4, gray);

    let day_name = Weekday::try_from(day_seperator as u8).unwrap().to_string();
    drawing::draw_text_mut(
      &mut img,
      white,
      start_x as i32 + 10,
      10 as i32,
      Scale { x: 30.0, y: 30.0 },
      &font,
      &day_name,
    );
  }

  dbg!(draw_time.elapsed());

  let save_time = std::time::Instant::now();
  img.save(file_path.as_str()).unwrap();
  dbg!(save_time.elapsed());
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

fn color_hash(course_code: &str) -> Rgba<u8> {
  let mut hasher = DefaultHasher::new();
  course_code.hash(&mut hasher);
  let hash = hasher.finish();

  let red = (hash & 0xFF) as u8;
  let green = ((hash >> 8) & 0xFF) as u8;
  let blue = ((hash >> 16) & 0xFF) as u8;
  Rgba([red, green, blue, 255])
}
