use std::str::FromStr;
use common::{
  read_file
};

fn p1(items: &Vec<String>){
  let mut horizontal: i64 = 0;
  let mut depth: i64 = 0;
  for item in items {
    let parts: Vec<_> = item.split(' ').collect();
    let direction = parts[0];
    let value = parts[1].parse::<i64>().unwrap();
    if direction.eq("forward"){
      horizontal += value
    }
    else if direction.eq("down"){
      depth += value
    }
    else if direction.eq("up"){
      depth -= value
    }
  }
  println!("Horizontal: {}", horizontal);
  println!("Depth: {}", depth);
  println!("Final value: {}", horizontal * depth);
}

fn p2(items: &Vec<String>){
  let mut horizontal: i64 = 0;
  let mut depth: i64 = 0;
  let mut aim: i64 = 0;
  for item in items {
    let parts: Vec<_> = item.split(' ').collect();
    let direction = parts[0];
    let value = parts[1].parse::<i64>().unwrap();
    if direction.eq("forward"){
      horizontal += value;
      depth += aim * value
    }
    else if direction.eq("down"){
      aim += value
    }
    else if direction.eq("up"){
      aim -= value
    }
  }
  println!("Horizontal: {}", horizontal);
  println!("Depth: {}", depth);
  println!("Aim: {}", aim);
  println!("Final value: {}", horizontal * depth);
}

fn main() {
    let items: Vec<String> = read_file("aoc2021/files/day2-input.txt");
    p1(&items); // pass vector as is.
    p2(&items); // pass vector as is.
}