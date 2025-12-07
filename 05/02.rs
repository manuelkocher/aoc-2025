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
  let mut sum_fresh: i64 = 0; // sum of fresh ingredients
  let input: &str = "./input.txt"; // file name of input

  if let Ok(lines) = read_lines(input) {
    let mut nums: Vec<Vec<i64>> = Vec::new();
    for line in lines.map_while(Result::ok) {
      if !line.contains("-") {
        continue;
      }
      let mut range: Vec<i64> = vec![0,0];
      for (i, num_str) in line.split("-").enumerate() {
        range[i] = num_str.parse::<i64>().unwrap();
      }
      nums.push(range.clone());
    }

    // iterate over nums, but with while to not burrow a value (it wouldnt allow pushing to the vec)
    let mut i: usize = 0;
    while i < nums.len() {
      // if a range changes, we need to check all previously checked again
      let mut changed: bool = true;
      while changed {
        changed = false;
        let mut range = nums[i].clone();
        let mut j: usize = 0;
        while j < i {
          let item = nums[j].clone();
          //println!("{} {} {} {} {} {}", range[0], range[1], item[0], item[1], j, i);
          // lower value is already covered, set it to upper bound +1
          if range[0] <= item[1] && range[0] >= item[0] {
            //println!("range[0] = {}", item[1] + 1);
            range[0] = item[1] + 1;
            changed = true;
          }
          // higher value is already covered, set it to lower bound -1
          if range[1] <= item[1] && range[1] >= item[0] {
            //println!("range[1] = {}", item[0] - 1);
            range[1] = item[0] - 1;
            changed = true;
          }
          j += 1;
          nums[i] = range.clone();
          //println!("{} {} {} {}:{}", nums[i][0], nums[i][1], item[0], item[1], i);
          //println!("{} < {} && {} > {}", range[0], item[0], range[1], item[1]);
          if range[0] < item[0] && range[1] > item[1] {
            nums.push(vec![range[0], item[0]-1]);
            nums.push(vec![item[1]+1, range[1]]);
            nums[i] = vec![1, 0]; // invalidate field
            changed = true;
            break;
          }
        }
      }
      i += 1;
    }
    for item in &nums {
      if item[1] < item[0] {
        continue;
      }
      //println!("{}-{} = + {}", item[0], item[1], item[1] - item[0] + 1);
      sum_fresh += item[1] - item[0] + 1;
    }  
  }

  println!("{}", sum_fresh);

  // wait for user input
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).unwrap();

  Ok(())
}