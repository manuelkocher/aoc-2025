use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone)]
struct Point {
    x: f64,
    y: f64,
    z: f64
}

fn distance(p1: Point, p2: Point) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2) + (p1.z - p2.z).powi(2)).sqrt()
}

// read a file by lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn main() -> io::Result<()> {
  let input: &str = "./input.txt"; // file name of input

  // save junctions and circuits as vector of point indexex and vector of vector of point indexes resp.
  let mut circuits: Vec<Vec<usize>> = Vec::new();
  let mut junctions: Vec<Point> = Vec::new();

  // for some reason we have a limit of the first 1000 lines in the files
  let limit = 1000;

  if let Ok(lines) = read_lines(input) {
    for (i, line) in lines.map_while(Result::ok).enumerate() {
        let coords: Vec<f64> = line.split(',').map(|s| s.parse::<f64>().unwrap()).collect();
        let point = Point{ x:coords[0], y:coords[1], z:coords[2] };
        junctions.push(point);
    }

    let mut counter: usize = 0;
    'limit_loop: while counter < limit {
      counter = counter + 1;
      println!("circuits: {:?}", circuits);
      // save minimum distance and indexes of points with minimum distance
      let mut min_dist = f64::MAX;
      let mut min_dist_points: Vec<usize> = Vec::new();
      for (i, point1) in junctions.clone().into_iter().enumerate() {
        min_dist = f64::MAX;
        min_dist_points = Vec::new();
        'junction_loop: for (j, point2) in junctions.clone().into_iter().enumerate() {
          if i == j {
            continue;
          }
          for circuit in &circuits {
            if circuit.contains(&i) && circuit.contains(&j) {
              continue 'junction_loop;
            }
          }
          let dist = distance(point1.clone(), point2.clone());
          if dist < min_dist {
            min_dist = dist;
            min_dist_points = vec![i, j];
            min_dist_points.sort();
          }
        }
      }

      // there are 2 cases:
      // 1. the two points with the least distance build a new circuit, so we just push the new circuit
      // 2. one of the two points is already part of a circuit, so we just need to push the other point

      // 1.
      for (k, point) in min_dist_points.clone().into_iter().enumerate() {
        for circuit in &mut circuits {
          // check if circuit already exists
          if circuit.contains(&point) {
            // add other point to existing circuit
            circuit.push(min_dist_points[(k + 1) % 2]);
              
            // due to lazyness, handle duplicates by dedup-ing
            circuit.sort();
            circuit.dedup();

            println!("push1 {:?}", circuit);

            // continue outer to loop, otherwise we would go to 2.
            continue 'limit_loop;
          }
        }
      }

      // 2.
      circuits.push(min_dist_points.clone());
      println!("push2 {:?}", min_dist_points);
    }
  }

  // go over circuits, get sizes and multiply them
  let product = circuits.iter().map(|inner| inner.len()).product::<usize>();
  println!("circuits: {:?}", circuits);
  println!("{}", product);

  // wait for user input
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).unwrap();

  Ok(())
}