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

            /* everything with less than 4 chars is handled here */
            if num_str.len() < 4 && num_str.len() > 1 {
              // check if all digits are the same, the only case where an uneven number is possible
              let first_ch = num_str.split_at(1).0;
              let mut all_same = true;
              for ch in num_str.chars() {
                if first_ch != ch.to_string() {
                  all_same = false;
                  break;
                }
              }
              if all_same {
                sum_ids += num;
              }
              continue;
            }
            
            /* everything greater 4 chars is handled here */
            let mut divisor = 2;
            let mut same = false;
            while num_str.len() / 2 >= divisor {
              if num_str.len() % divisor != 0 {
                // check if all digits are the same, the only case where an uneven number is possible
                let first_ch = num_str.split_at(1).0;
                let mut all_same = true;
                for ch in num_str.chars() {
                  if first_ch != ch.to_string() {
                    all_same = false;
                    break;
                  }
                }
                if all_same {
                  sum_ids += num;
                  break;
                }
                divisor += 1;
                continue;
              } 

              same = true;
              let part_size = num_str.len() / divisor;
              let mut part_slice = 0;
              while part_slice < num_str.len() - part_size {
                let first = &num_str[part_slice..part_size+part_slice];
                let second = &num_str[part_slice+part_size..(2*part_size)+part_slice];
                // we assume that its always numbers due to the task description
                // thus no need to handle edge cases
                if first.parse::<i64>().unwrap() != second.parse::<i64>().unwrap() {
                  same = false;
                } 
                part_slice += part_size;
              }
              if same {
                break;
              }
              divisor += 1;
            }
            if same {
              sum_ids += num;
            }
          }
        }
      }
  }
  println!("\n{}", sum_ids);

  // wait for user input
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).unwrap();
}