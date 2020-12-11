extern crate regex;

pub mod tools;
pub mod aoc_1st_day;
pub mod aoc_2nd_day;
pub mod aoc_3rd_day;
pub mod aoc_4th_day;
pub mod aoc_5th_day;
pub mod aoc_6th_day;
pub mod aoc_7th_day;
pub mod aoc_8th_day;

fn main() {
  //first_day();
  //second_day();
  //third_day();
  //fourth_day();
  //fifth_day();
  //sixth_day();
  //seventh_day();
  eighth_day();
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

#[allow(dead_code)]
fn sixth_day() {
  println!(
    "Sixth day first part value: {:?}",
    aoc_6th_day::custom_customs::compute1()
  );

  println!(
    "Sixth day second part value: {:?}",
    aoc_6th_day::custom_customs::compute2()
  );
}

#[allow(dead_code)]
fn seventh_day() {
  println!(
    "Seventh day first part value: {:?}",
    aoc_7th_day::handy_haversacks::compute1()
  );

  println!(
    "Seventh day second part value: {:?}",
    aoc_7th_day::handy_haversacks::compute2()
  );
}

#[allow(dead_code)]
fn eighth_day() {
  println!(
    "Eighth day first part value: {:?}",
    aoc_8th_day::handheld_halting::compute1()
  );

  println!(
    "Eighth day second part value: {:?}",
    aoc_8th_day::handheld_halting::compute2()
  );
}
