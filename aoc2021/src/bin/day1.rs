use std::path::PathBuf;
use common::{
  get_input_file_pathbuf,
  read_file_return_vec
};

fn part1_count(numbers: &[i64]){
    let mut count = 0;
    for x in 0..numbers.len()-2 {
      if numbers[x+1] > numbers[x] {
        count += 1
      }
    }

    println!("Part1 count: {}", count)
}

fn part2_count(numbers: &mut Vec<i64>){
  // Creating another vector with 3 zeros to append to original vector. This new vector needs to be mutable as well in order to be appended.
  let mut filler: Vec<i64> = vec![0; 3];
  numbers.append(&mut filler);
  
  // Iterate until last element of original vector (before filler was appeneded).
  let mut count = 0;
  for x in 0..numbers.len()-4 {
    if (numbers[x] + numbers[x+1] + numbers[x+2]) < (numbers[x+1] + numbers[x+2] + numbers[x+3]) {
      count += 1
    }
  }

  println!("Part2 count: {}", count)
}

fn main() {
    let path: PathBuf = get_input_file_pathbuf("aoc2021/files/day1-input.txt");
    let mut numbers: Vec<i64> = read_file_return_vec(&path); // creating this as mutable since we want to add elements to it in part2
    part1_count(&numbers); // pass vector as is.
    part2_count(&mut numbers); // pass mutable vector as elements will be added to it.
}