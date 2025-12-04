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
  let mut pw: i32 = 0; // "calculated" password
  let input: &str = "./input.txt"; // file name of input
  let debug: bool = false;

  if let Ok(lines) = read_lines(input) {
    for line in lines.map_while(Result::ok) {
      let new_pos: i32;
      let mut zero_rot: i32;
      let num: i32;
      num = line[1..].parse::<i32>().expect("Not a number");
      
      if line[0..1] == *"L" {
        new_pos = true_mod(dial - num, 100);
        zero_rot = (dial - num).abs() / 100;
        if dial < num && dial != 0 {
          zero_rot += 1;
        }
        if debug {
          println!("{} - {}", dial, num);
        }
      } else {
        new_pos = true_mod(dial + num, 100);
        zero_rot = (dial + num) / 100;
        if debug {
          println!("{} + {}", dial, num);
        }
      }
      zero_rot = zero_rot.abs();
      if debug {
        println!("zero rot: {}", zero_rot);
        println!("new pos: {}", new_pos);
      }
      if zero_rot > 0 {
        pw += zero_rot;
      } else if new_pos == 0 && zero_rot == 0 {
        pw += 1;
      }
      if debug {
        println!("pw: {}", pw);
      }

      dial = new_pos;
    }
  }
  println!("dial: {}", dial);
  println!("{}", pw);

  // wait for user input
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).unwrap();
}