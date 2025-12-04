use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// read a file by lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut sum_ids: i64 = 0; // sum of invalid IDs
    let input: &str = "./input.txt"; // file name of input

    if let Ok(lines) = read_lines(input) {
      for line in lines.map_while(Result::ok) {
        let raw_ranges: Vec<&str> = line.split(',').collect();
        for raw_range in raw_ranges {
          let range: Vec<&str> = raw_range.split('-').collect();
          // we assume that every range is in format X-Y due to the task description
          // thus no need to check for edge cases
          let start = range[0].parse::<i64>().unwrap();
          let end = range[1].parse::<i64>().unwrap() + 1;
          for num in start..end {
            let num_str = num.to_string();
            if num_str.len() % 2 != 0 {
              continue;
            } 
            let (first, second) = num_str.split_at(num_str.len() / 2);
            // we assume that its always numbers due to the task description
            // thus no need to handle edge cases
            if first.parse::<i64>().unwrap() == second.parse::<i64>().unwrap() {
              //println!("{}", num);
              sum_ids += num;
            } 
          }
        }
      }
  }
  println!("{}", sum_ids);

  // wait for user input
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).unwrap();
}