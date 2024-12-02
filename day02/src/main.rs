// Day 02: Red-Nosed Reports

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

const FILE_NAME: &str = "./data.txt";

enum Direction {
  Unknown,
  Increasing,
  Decreasing
}

fn solve() {
  println!("Day 02: Part 1");
  println!("GOAL: Analyze the unusual data from the engineers.");
  println!("ANSWER: How many reports are safe?");

  let mut valid_reports = 0;

  let re_split = Regex::new(r"(\s+)").expect("Invalid regex");
    if let Ok(lines) = read_lines(FILE_NAME) {
    for line in lines.flatten() {
      let values: Vec<_> = re_split.split(&line).map(|f| f.parse::<u32>().unwrap()).collect();

      let is_valid = is_valid_report(&values);

      // println!("Report {} {} valid", line, if is_valid { "is" } else { "is NOT" });

      if is_valid {
        valid_reports += 1;
      }
    }
  }
  println!("Valid reports = {valid_reports}");

  println!("-----------");
  println!("Day 02: Part 2");
  println!("GOAL: Update your analysis by handling situations where the Problem Dampener can remove a single level from unsafe reports.");
  println!("ANSWER: How many reports are now safe?");

  valid_reports = 0;
  if let Ok(lines) = read_lines(FILE_NAME) {
    for line in lines.flatten() {
      let values: Vec<_> = re_split.split(&line).map(|f| f.parse::<u32>().unwrap()).collect();

      let is_valid = is_any_valid_report(&values);

      println!("Report {} {} valid", line, if is_valid { "is" } else { "is NOT" });

      if is_valid {
        valid_reports += 1;
      }
    }
  }  
  println!("Valid reports = {valid_reports}");

}

fn is_valid_report (values: &Vec<u32>) -> bool {
  let mut direction = Direction::Unknown;
  // println!("Checking values {:?}", values);
  let mut last_report = *values.first().unwrap();

  for r in &values[1..] {
    let report = *r;

    if matches!(direction, Direction::Unknown) {
      direction =
      if report > last_report { Direction::Increasing }
      else { Direction::Decreasing };
    }

    if report == last_report { return false; };
    if matches!(direction, Direction::Increasing) && (report < last_report || report > last_report + 3) { return false };
    if matches!(direction, Direction::Decreasing) && (report > last_report || report + 3 < last_report) { return false };

    last_report = report;
  };

  return true;
}

fn is_any_valid_report (values: &Vec<u32>) -> bool {
  if is_valid_report(values) { return true; }
  let length = values.len();
  for idx in 0..length {
    if is_valid_report(&copy_except(values, idx)) { return true; }
  }

  return false;
}

fn copy_except (values: &Vec<u32>, exclude_idx: usize) -> Vec<u32> {
  let mut new_values: Vec<u32> = Vec::new();
  let mut idx = 0;
  for value in values {
    if idx != exclude_idx {
      new_values.push(*value);
    }
    idx += 1;
  }
  // println!("copied new values {:?}", new_values);
  return new_values;
}

fn main() {
  solve();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
