extern crate regex;

pub mod tools;
pub mod aoc_1st_day;
pub mod aoc_2nd_day;
pub mod aoc_3rd_day;
pub mod aoc_4th_day;
pub mod aoc_5th_day;

fn main() {
  //first_day();
  //second_day();
  //third_day();
  //fourth_day();
  fifth_day()
}

#[allow(dead_code)]
fn first_day() {
  println!(
    "First day first part value: {:?}",
    aoc_1st_day::report_repair::compute1()
  );

  println!(
    "First day second part value: {:?}",
    aoc_1st_day::report_repair::compute2()
  );
}

#[allow(dead_code)]
fn second_day() {
  println!(
    "Second day first part value: {:?}",
    aoc_2nd_day::password_philosophy::compute1()
  );
  println!(
    "Second day second part value: {:?}",
    aoc_2nd_day::password_philosophy::compute2()
  );
}

#[allow(dead_code)]
fn third_day() {
  println!(
    "Third day first part value: {:?}",
    aoc_3rd_day::toboggan_trajectory::compute1()
  );
  println!(
    "Third day second part value: {:?}",
    aoc_3rd_day::toboggan_trajectory::compute2()
  );
}

#[allow(dead_code)]
fn fourth_day() {
  println!(
    "Fourth day first part value: {:?}",
    aoc_4th_day::passport_processing::compute1()
  );

  println!(
    "Fourth day second part value: {:?}",
    aoc_4th_day::passport_processing::compute2()
  );
}

#[allow(dead_code)]
fn fifth_day() {
  println!(
    "Fifth day first part value: {:?}",
    aoc_5th_day::binary_boarding::compute1()
  );

  println!(
    "Fifth day second part value: {:?}",
    aoc_5th_day::binary_boarding::compute2()
  );
}
