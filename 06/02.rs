use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// read a file by lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn calc_intermediate_result(inverted_nums: &mut Vec<i64>, current_op: char, results: &mut Vec<i64>)
{
  let mut res: i64;
  if current_op == '*' {
    res = 1;
  } else {
    res = 0;
  }
  for num in &mut *inverted_nums {
    if current_op == '*' {
      res *= *num;
    } else {
      res += *num;
    }
  }
  results.push(res.clone());
  *inverted_nums = Vec::new();
}

fn main() -> io::Result<()> {
  let sum_results: i64; // sum of results
  let input: &str = "./input.txt"; // file name of input

  let mut num_strs: Vec<String> = Vec::new();
  let mut ops: Vec<char> = Vec::new();
  let mut results: Vec<i64> = Vec::new();
  let mut inverted_nums: Vec<i64> = Vec::new();
  let mut current_op: char = '+';

  // we could have combined both loops, but i switched to vec midway and didnt bother to rewrite everything
  if let Ok(lines) = read_lines(input) {
    for line in lines.map_while(Result::ok) {
      // we can assume that at pos 0 theres always a char present
      // if first char is a operator, break loop
      let c = line.chars().nth(0).unwrap();
      if c == '*' || c == '+' {
        ops = line.chars().collect::<Vec<char>>().clone();
        break;
      }
      num_strs.push(line.clone());
    }
  }

  for (i, op) in ops.iter().enumerate() {
    if *op == '+' || *op == '*' {
      current_op = op.clone();
    }
    // skip empty column
    // empty columns always occur when the next char is a new operator
    if i+1 < ops.len() && (ops[i+1] == '*' || ops[i+1] == '+') {
      calc_intermediate_result(&mut inverted_nums, current_op, &mut results);
      continue;
    }
    let mut res = String::new();
    for line in &num_strs {
      let c = &line.chars().nth(i).unwrap().to_string();
      if c == " " {
        continue;
      }
      res.push_str(&c.clone());
    }
    //println!("{}: {}",i, res);
    inverted_nums.push(res.parse::<i64>().unwrap());
  }
  calc_intermediate_result(&mut inverted_nums, current_op, &mut results);
  //println!("{:?}", results);
  sum_results = results.iter().sum();

  println!("{}", sum_results);

  // wait for user input
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).unwrap();

  Ok(())
}