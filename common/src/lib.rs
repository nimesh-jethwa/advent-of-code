use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use project_root;
use std::path::Path;
use std::path::PathBuf;

pub fn read_file_return_vec(file_path: &PathBuf) -> Vec<i64> {
    let file = File::open(file_path).expect("File not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect()
}

pub fn get_input_file_pathbuf(input_file: &str) -> PathBuf {
  [project_root::get_project_root().unwrap(), Path::new(input_file).to_path_buf()].iter().collect() // construct a path to input file
}