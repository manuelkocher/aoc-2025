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
  let mut sum_rolls: i64 = 0; // sum of volts
  let input: &str = "./input.txt"; // file name of input

  // get line count and number of chars to init the grid
  let mut file = File::open(input)?;
  let mut reader = io::BufReader::new(file);
  let line_count = reader.lines().count();
  file = File::open(input)?;
  reader = io::BufReader::new(file);
  let mut first_line = String::new();
  reader.read_line(&mut first_line)?;


  let mut grid: Vec<Vec<i32>> = vec![vec![0; line_count]; first_line.len() -2]; // -2 because \c\n

  // we could have combined both loops, but i switched to vec midway and didnt bother to rewrite everything
  if let Ok(lines) = read_lines(input) {
    let mut line_idx = 0;
    for line in lines.map_while(Result::ok) {
      for (i, c) in line.chars().enumerate() {
        if c == '@' {
          grid[line_idx][i] = 1;
        }
      }
      line_idx += 1;
    }
  }

  for y in 0..grid.len() {
    for x in 0..grid[y].len() {
      // only count items where a @ is set
      if grid[y][x] != 1 {
        continue;
      }
      // we could hardcode that with grid[y][x-1], grid[y][x+1] etc if cases, but i dont like to
      // so we iterate (again)
      let mut rolls = 0;
      for i in -1isize..2 {
        for j in -1isize..2 {
          let yi = y as i32;
          let xi = x as i32;
          let ii = i as i32;
          let ji = j as i32;
          if (yi + ii < 0) || yi+ii >= grid.len() as i32 || xi+ji < 0 || xi+ji >= grid[y].len() as i32 || (ii == 0 && ji == 0) {
            continue;
          }
          
          if grid[(yi+ii) as usize][(xi+ji) as usize] == 1 {
            rolls += 1;
            //println!("+1 {} {} {} {}", yi, ii, xi, ji);
          }
        }
      }
      if rolls < 4 {
        sum_rolls += 1;
        //println!("sum_rolls: {}, grid[{}][{}]={}", sum_rolls, y, x, grid[y][x]);
      }
    }
  }

  println!("{}", sum_rolls);

  // wait for user input
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).unwrap();

  Ok(())
}