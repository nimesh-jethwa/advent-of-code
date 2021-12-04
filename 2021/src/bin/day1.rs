use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use project_root;
use std::path::Path;
use std::path::PathBuf;

fn read_file(file_path: &PathBuf) -> Vec<i64> {
    let file = File::open(file_path).expect("File not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect()
}

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
    let path: PathBuf = [project_root::get_project_root().unwrap(), Path::new("2021/files/day1-input.txt").to_path_buf()].iter().collect(); // construct a path to input file
    let mut numbers: Vec<i64> = read_file(&path); // creating this as mutable since we want to add elements to it in part2
    part1_count(&numbers); // pass vector as is.
    part2_count(&mut numbers); // pass mutable vector as elements will be added to it.
}