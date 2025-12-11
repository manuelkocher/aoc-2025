use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// read a file by lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}


fn main() -> io::Result<()> {
  let mut sum_results: i64 = 0; // sum of results
  let input: &str = "./input.txt"; // file name of input

  // we could have combined both loops, but i switched to vec midway and didnt bother to rewrite everything
  if let Ok(lines) = read_lines(input) {
    let mut nums: Vec<Vec<i64>> = Vec::new();
    for line in lines.map_while(Result::ok) {
      let items_in_line: Vec<&str> = line.split_whitespace().collect();
      let result: Result<Vec<i64>, _> = items_in_line.iter().map(|n| n.parse()).collect();

      match result {
        // if parsing worked, remember numbers
        Ok(numbers) => {
          println!("{:?}", numbers);
          nums.push(numbers.clone());
        }
        // otherwise we assume that the line is operators and we calc the result
        Err(_) => {
          let mut results: Vec<i64> = Vec::new();
          let mut j: usize = 0;
          for op in &items_in_line {
            let mut res: i64 = 0;
            for i in 0..nums.len() {
              match *op {
                "*" => {
                  if i == 0 {
                    res += 1; // 1 is the neutral element for *
                  }
                  res *= nums[i][j];
                },
                "+" => {
                  res += nums[i][j];
                },
                _ => println!("Other"),
              }
            }
            j += 1;
            results.push(res);
          }
          println!("{:?}", results);
          sum_results = results.iter().sum();
        }
      }
    }
  }

  println!("{}", sum_results);

  // wait for user input
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).unwrap();

  Ok(())
}