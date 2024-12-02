// Day 01: Historian Hysteria

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn solve_part1() {
  println!("Day 01: Part 1");
  println!("GOAL: Find the total distance between the left list and the right list");
  println!("ANSWER: What is the total distance between your lists?");

  let mut vec_a = Vec::with_capacity(1000);
  let mut vec_b = Vec::with_capacity(1000);
  let mut result = 0;
  let re_split = Regex::new(r"(\s+)").expect("Invalid regex");
  if let Ok(lines) = read_lines("./data.txt") {
    for line in lines.flatten() {
      let vals: Vec<&str> = re_split.split(&line).collect();

      // println!("{} and {}", vals[0], vals[1]);
      let a = vals[0].parse::<i32>().unwrap();
      let b = vals[1].parse::<i32>().unwrap();
      vec_a.push(a);
      vec_b.push(b);
    }
    vec_a.sort();
    vec_b.sort();

    for i in 0..1000 {
      let diff = vec_a[i].abs_diff(vec_b[i]);
      result += diff;
      println!("Line {}: abs({} - {}) = {}", i+1, vec_a[i], vec_b[i], diff)
    }
  }
  println!("Total {result}");
}

fn solve_part2 () {
  println!("Day 01: Part 2");
  println!("GOAL: #GOAL2#");
  println!("ANSWER: #ANSWER2#");
}

fn main() {
  solve_part1()
  // solve_part2()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}