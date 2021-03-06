use common::{
  convert_vector_string_to_i64,
  read_file
};

fn p1(numbers: &Vec<i64>){
    let mut count = 0;
    for x in 0..numbers.len()-2 {
      if numbers[x+1] > numbers[x] {
        count += 1
      }
    }

    println!("Part1 count: {}", count)
}

fn p2(numbers: &mut Vec<i64>){
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
    let mut items: Vec<i64> = convert_vector_string_to_i64(read_file("aoc2021/files/day1-input.txt"));
    p1(&items); // pass vector as is.
    p2(&mut items); // pass mutable vector as elements will be added to it.
}