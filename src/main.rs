mod data;
mod sample_data;

fn main() {
  let subjects = sample_data::get_subjects();
  println!("Subjects: {:?}", subjects);
}
