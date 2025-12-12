use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

// read a file by lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn find_S(line: &str) -> usize
{
  for (i, c) in line.chars().enumerate() {
    if c == 'S' {
      return i;
    }
  }
  return 0;
}

fn main() -> io::Result<()> {
  let mut num_splits: i64 = 0; // number of splits
  let input: &str = "./input.txt"; // file name of input

  let mut beam_idxs: Vec<usize> = Vec::new();
  let mut map = HashMap::new();

  if let Ok(lines) = read_lines(input) {
    for (i, line) in lines.map_while(Result::ok).enumerate() {
      if i == 0 {
        beam_idxs.push(find_S(&line));
      } else {
        // go over all beams and split whenever a ^ is present
        let new_beam_idxs: Vec<usize> = Vec::new();
        let mut asterisk_idxs: Vec<usize> = Vec::new();
        for idx in beam_idxs.clone() {
          let c = line.chars().nth(idx).unwrap();
          if c == '^' {
            let mut init = 1;
            if map.contains_key(&idx) {
              init = *map.get(&idx).unwrap();
            }
            if beam_idxs.contains(&(idx - 1)) {
              let val = map.get_mut(&(idx-1)).unwrap();
              *val += init;
            } else {
              map.insert(idx-1, init);
              beam_idxs.push(idx - 1);
            }

            if beam_idxs.contains(&(idx + 1)) {
              let val = map.get_mut(&(idx + 1)).unwrap();
              *val += init;
            } else {
              map.insert(idx+1, init);
              beam_idxs.push(idx + 1);
            }
            num_splits += 1;
            //println!("push {} {}, remove {:?}", idx - 1, idx + 1, beam_idxs.iter().position(|n| *n == idx));
            beam_idxs.remove(beam_idxs.iter().position(|n| *n == idx).unwrap()); 
            if !asterisk_idxs.contains(&idx) {
              asterisk_idxs.push(idx);
            }
          }
        }
        for idx in asterisk_idxs {
          if map.contains_key(&idx) {
            map.remove(&idx);
          }
        }
        beam_idxs.sort();
        beam_idxs.dedup();
        //println!("{:?}", beam_idxs);
        //println!("{}: {}", i, beam_idxs.len());
      }
    }
  }

  println!("\n{}", map.values().sum::<usize>());

  // wait for user input
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).unwrap();

  Ok(())
}