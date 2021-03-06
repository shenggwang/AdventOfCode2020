use std::io::{stdin};
extern crate regex;

pub mod tools;
pub mod aoc_0th_day;
pub mod aoc_1st_day;
pub mod aoc_2nd_day;
pub mod aoc_3rd_day;
pub mod aoc_4th_day;
pub mod aoc_5th_day;
pub mod aoc_6th_day;
pub mod aoc_7th_day;
pub mod aoc_8th_day;
pub mod aoc_9th_day;
pub mod aoc_10th_day;
pub mod aoc_11th_day;
pub mod aoc_12th_day;
pub mod aoc_13th_day;
pub mod aoc_14th_day;
pub mod aoc_15th_day;
pub mod aoc_16th_day;
pub mod aoc_17th_day;
pub mod aoc_18th_day;
pub mod aoc_19th_day;

fn main() {
  let mut buffer = String::new();
  println!("Choose between 1 until 19 to get the output:");
  stdin().read_line(&mut buffer).expect("Failed reading line");
  let input = buffer.trim().parse::<u8>().unwrap();
  match input {
    0 => zeroth_day(),
    1 => first_day(),
    2 => second_day(),
    3 => third_day(),
    4 => fourth_day(),
    5 => fifth_day(),
    6 => sixth_day(),
    7 => seventh_day(),
    8 => eighth_day(),
    9 => nineth_day(),
    10 => tenth_day(),
    11 => eleventh_day(),
    12 => twelfth_day(),
    13 => thirteenth_day(),
    14 => fourteenth_day(),
    15 => fifteenth_day(),
    16 => sixteenth_day(),
    17 => seventeenth_day(),
    18 => eighteenth_day(),
    19 => nineteenth_day(),
    _ => println!("Option {} not found", input),
  }
}

#[allow(dead_code)]
fn zeroth_day() {
  let mut buffer = String::new();
  println!("Zeroth day: choose 1- normal calculation, 2- fee and yearly deposit calculation");
  stdin().read_line(&mut buffer).expect("Failed reading line");
  let input = buffer.trim().parse::<u8>().unwrap();
  match input {
    1 =>  println!(
            "Zeroth day first part value: {:?}",
            aoc_0th_day::cumulative_calculation::compute1()
          ),
    2 =>  println!(
            "Zeroth day second part value: {:?}",
            aoc_0th_day::cumulative_calculation::compute2()
          ),
    _ => println!("Option {} not found", input),
  }
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

#[allow(dead_code)]
fn nineth_day() {
  println!(
    "Nineth day first part value: {:?}",
    aoc_9th_day::encoding_error::compute1()
  );

  println!(
    "Nineth day second part value: {:?}",
    aoc_9th_day::encoding_error::compute2()
  );
}

#[allow(dead_code)]
fn tenth_day() {
  println!(
    "Tenth day first part value: {:?}",
    aoc_10th_day::adapter_array::compute1()
  );

  println!(
    "Tenth day second part value: {:?}",
    aoc_10th_day::adapter_array::compute2()
  );
}

#[allow(dead_code)]
fn eleventh_day() {
  println!(
    "Eleventh day first part value: {:?}",
    aoc_11th_day::seating_system::compute1()
  );

  println!(
    "Eleventh day second part value: {:?}",
    aoc_11th_day::seating_system::compute2()
  );
}

#[allow(dead_code)]
fn twelfth_day() {
  println!(
    "Twelfth day first part value: {:?}",
    aoc_12th_day::rain_risk::compute1()
  );

  println!(
    "Twelfth day second part value: {:?}",
    aoc_12th_day::rain_risk::compute2()
  );
}

#[allow(dead_code)]
fn thirteenth_day() {
  println!(
    "Thirteenth day first part value: {:?}",
    aoc_13th_day::shuttle_search::compute1()
  );

  println!(
    "Thirteenth day second part value: {:?}",
    aoc_13th_day::shuttle_search::compute2()
  );
}

#[allow(dead_code)]
fn fourteenth_day() {
  println!(
    "Fourteenth day first part value: {:?}",
    aoc_14th_day::docking_data::compute1()
  );

  println!(
    "Fourteenth day second part value: {:?}",
    aoc_14th_day::docking_data::compute2()
  );
}

#[allow(dead_code)]
fn fifteenth_day() {
  println!(
    "Fifteenth day first part value: {:?}",
    aoc_15th_day::rambunctious_recitation::compute1()
  );

  println!(
    "Fifteenth day second part value: {:?}",
    aoc_15th_day::rambunctious_recitation::compute2()
  );
}

#[allow(dead_code)]
fn sixteenth_day() {
  println!(
    "Sixteenth day first part value: {:?}",
    aoc_16th_day::ticket_translation::compute1()
  );

  println!(
    "Sixteenth day second part value: {:?}",
    aoc_16th_day::ticket_translation::compute2()
  );
}

#[allow(dead_code)]
fn seventeenth_day() {
  println!(
    "Seventeenth day first part value: {:?}",
    aoc_17th_day::conwey_cube::compute1()
  );

  println!(
    "Seventeenth day second part value: {:?}",
    aoc_17th_day::conwey_cube::compute2()
  );
}

#[allow(dead_code)]
fn eighteenth_day() {
  println!(
    "Eighteenth day first part value: {:?}",
    aoc_18th_day::operation_order::compute1()
  );

  println!(
    "Eighteenth day second part value: {:?}",
    aoc_18th_day::operation_order::compute2()
  );
}

#[allow(dead_code)]
fn nineteenth_day() {
  /*
  println!(
    "Nineteenth day first part value: {:?}",
    aoc_19th_day::monster_messages::compute1()
  );
  */
  println!(
    "Nineteenth day second part value: {:?}",
    aoc_19th_day::monster_messages::compute2()
  );
}
