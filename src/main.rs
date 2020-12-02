pub mod tools;
pub mod first_day;

fn first_day() {
  println!(
    "First day first part value: {:?}", 
    first_day::report_repair::compute1()
  );

  println!(
    "First day second part value: {:?}", 
    first_day::report_repair::compute2()
  );
}

fn main() {
  println!(
    "{aoc} day playing {day}",
    aoc = "Advent Of Code",
    day = "1"
  );

  first_day()

}
