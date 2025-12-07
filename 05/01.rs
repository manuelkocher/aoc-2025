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
  let mut sum_fresh: i64 = 0; // sum of volts
  let input: &str = "./input.txt"; // file name of input

  // we could have combined both loops, but i switched to vec midway and didnt bother to rewrite everything
  if let Ok(lines) = read_lines(input) {
    let mut nums: Vec<Vec<i64>> = Vec::new();
    'lines: for line in lines.map_while(Result::ok) {
      if line.trim().is_empty() {
        continue;
      }
      if line.contains("-") {
        let mut range: Vec<i64> = vec![0,0];
        for (i, num_str) in line.split("-").enumerate() {
          range[i] = num_str.parse::<i64>().unwrap();
        }
        nums.push(range.clone());
      }
      else {
        //println!("{}", line);
        let fresh = line.parse::<i64>().unwrap();
        for range in &nums {
          if fresh >= range[0] && fresh <= range[1] {
            //println!("fresh: {}", fresh);
            sum_fresh += 1;
            continue 'lines;
          }
        }
      }
    }
  }

  println!("{}", sum_fresh);

  // wait for user input
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).unwrap();

  Ok(())
}