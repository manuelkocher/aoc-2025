use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// real modulo, not remainder
fn true_mod(a: i32, b: i32) -> i32 {
  ((a % b) + b) % b
}

// read a file by lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut dial: i32 = 50; // default dial position
    let mut pw: u32 = 0; // "calculated" password
    let input: &str = "./input.txt"; // file name of input

    if let Ok(lines) = read_lines(input) {
      for line in lines.map_while(Result::ok) {
        let new_pos: i32;
        if line[0..1] == *"L" {
          new_pos = true_mod(dial - line[1..].parse::<i32>().expect("Not a number"), 100);
        } else {
          new_pos = true_mod(dial + line[1..].parse::<i32>().expect("Not a number"), 100);
        }
        if  new_pos == 0 {
          pw += 1;
        }
        dial = new_pos;
      }
  }
  println!("{}", pw);

  // wait for user input
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).unwrap();
}