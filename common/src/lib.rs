use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use project_root;
use std::path::Path;
use std::path::PathBuf;

pub fn read_file(input_file: &str) -> Vec<String> {
  read_file_return_vec(&get_input_file_pathbuf(&input_file))
}

pub fn convert_vector_string_to_i64(v: Vec<String>) -> Vec<i64> {
  v.iter().flat_map(|x| x.parse()).collect()
}

pub fn get_input_file_pathbuf(input_file: &str) -> PathBuf {
  [project_root::get_project_root().unwrap(), Path::new(input_file).to_path_buf()].iter().collect() // construct a path to input file
}

pub fn read_file_return_vec(file_path: &PathBuf) -> Vec<String> {
    let file = File::open(file_path).expect("File not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect()
}