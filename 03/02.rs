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
  let mut sum_volts: i64 = 0; // sum of volts
  let input: &str = "./input.txt"; // file name of input

  let mut idx: [i32; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
  let mut linenum = 1;

  if let Ok(lines) = read_lines(input) {
    for line in lines.map_while(Result::ok) {
      let mut s = String::from("");
      for i in 0..idx.len() {
        'outer: for (j, c) in line.chars().enumerate() {
          if i > 0 && (j as i32) < idx[i-1] && idx[i-1] != (line.len() - 1) as i32 {
            continue 'outer;
          }
          let saved = line[idx[i] as usize .. idx[i] as usize + 1].parse::<i32>().unwrap();
          let current = c.to_digit(10).unwrap() as i32;
          //println!("{} {} {} {}", i, idx[i], saved, current);
          if saved < current && line.len() - j > idx.len() - 1 - i {
            //println!("{} {} {} {} {} {}", i, saved, current, linenum, j, idx[i]);
            for x in &idx[..i] {
              if *x == j as i32 {
                //println!("continue!");
                continue 'outer;
              }
            }
            idx[i] = j as i32;
            //println!("idx[{}]={}", i, j);
            if j+1 < line.len() && i+1 < idx.len() {
              for x in 0..idx.len() {
                if x <= i {
                  continue;
                }
                idx[x] = j as i32 + (x as i32 - i as i32);
              }
              //println!("idx[{}+1]={}", i, j as i32 + 1);
            }
          }
        }
        //println!("{}", idx[i]);
      }
      idx.sort();
      //println!("\n{} {}", idx[0], idx[1]);
      for i in 0..idx.len() {
        s += &line[idx[i] as usize .. idx[i] as usize + 1]
      }
      println!("{}: {}", linenum, s);
      sum_volts += s.parse::<i64>().unwrap();

      idx = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
      linenum += 1;

    }
  }

  println!("{}", sum_volts);

  // wait for user input
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).unwrap();
}