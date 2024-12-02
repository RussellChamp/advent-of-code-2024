// Day 01: Historian Hysteria

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn solve() {
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

      let a = vals[0].parse::<u32>().unwrap();
      let b = vals[1].parse::<u32>().unwrap();
      vec_a.push(a);
      vec_b.push(b);
    }
    vec_a.sort();
    vec_b.sort();

    for i in 0..1000 {
      let diff = vec_a[i].abs_diff(vec_b[i]);
      result += diff;
      // println!("Line {}: abs({} - {}) = {}", i+1, vec_a[i], vec_b[i], diff)
    }
    println!("Total {result}");

    println!("Day 01: Part 2");
    println!("GOAL: Calculate a total similarity score by adding up each number in the left list after multiplying it by the number of times that number appears in the right list");
    println!("ANSWER: What is their similarity score?");

    result = 0;
    for i in 0..1000 {
      let needle = vec_a[i];
      let found = vec_b.iter().filter(|&&b| b == needle).count() as u32;
      let similarity =  found * needle;
      result += similarity;

      println!("Line {}: {} found {} times = {}, total {}", i + 1, needle, found, similarity, result);
    }
    println!("Total {result}");
  }
}

fn main() {
  solve()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}