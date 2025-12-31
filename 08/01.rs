use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::time::Instant;
use std::collections::BTreeMap;
use std::cmp::Reverse;

#[derive(Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64
}

fn distance(p1: Point, p2: Point) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2) + (p1.z - p2.z).powi(2)).sqrt()
}

fn check_connection_other_circuit(current_circuit_index: usize, i: usize, j: usize, circuits: &Vec<HashSet<usize>>) -> usize {
  for (idx, circuit) in circuits.iter().enumerate() {
    if current_circuit_index == idx {
      continue;
    }
    if circuit.contains(&i) || circuit.contains(&j) {
      return idx;
    }
  }
  current_circuit_index
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
  let mut circuits: Vec<HashSet<usize>> = Vec::new();
  let mut junctions: Vec<Point> = Vec::new();

  // for some reason we have a limit of the first 1000 lines in the files
  let limit = 1000;

  if let Ok(lines) = read_lines(input) {
    for (i, line) in lines.map_while(Result::ok).enumerate() {
        let coords: Vec<f64> = line.split(',').map(|s| s.parse::<f64>().unwrap()).collect();
        let point = Point{ x:coords[0], y:coords[1], z:coords[2] };
        junctions.push(point);
    }
    let mut start = Instant::now();
    let mut distances: BTreeMap<(usize, usize), f64> = BTreeMap::new();

    for (i, point1) in junctions.clone().into_iter().enumerate() {
      'junction_loop: for (j, point2) in junctions.clone().into_iter().enumerate() {
        if j <= i {
          continue;
        }
        let dist = distance(point1.clone(), point2.clone());
        distances.insert((i, j), dist);
      }
    }
    println!("Took: {:?}", start.elapsed());
    start = Instant::now();

    let mut items: Vec<_> = distances.iter().collect();

    // Sort by value (f64) using total_cmp to handle all floats safely
    items.sort_by(|a, b| a.1.total_cmp(b.1));

    println!("Took: {:?}", start.elapsed());

    'outer: for item in items.iter().take(limit) {
      //println!("state: {:?}", circuits);
      for idx in 0..circuits.len() {
        let circuit = circuits[idx].clone();
        if circuit.contains(&item.0.0) {
          //println!("{:?}: add {} {:?} to {:?}", item.0, item.0.1, junctions[item.0.1], circuit);
          let check = check_connection_other_circuit(idx, item.0.0, item.0.1, &circuits);          
          circuits[idx].insert(item.0.1);
          if check != idx {
            let other = circuits[check].clone();
            circuits[idx].extend(other);
            circuits.remove(check);
          }
          continue 'outer;
        } else if circuit.contains(&item.0.1) {
          //println!("{:?}: add {} {:?} to {:?}", item.0, item.0.0, junctions[item.0.0], circuit);
          let check = check_connection_other_circuit(idx, item.0.0, item.0.1, &circuits);
          circuits[idx].insert(item.0.0);
          if check != idx {
            let other = circuits[check].clone();
            circuits[idx].extend(other);
            circuits.remove(check);
          }
          continue 'outer;
        }
      }
      circuits.push(HashSet::from([item.0.0, item.0.1]));
      //println!("push {:?}: {:?} {:?}", item.0, junctions[item.0.0], junctions[item.0.1]);
    }
    //println!("{:?}",circuits);
    // part 1
    // go over circuits, get sizes and multiply the biggest 3 of them
    circuits.sort_by_key(|a| Reverse(a.len()));
    let product = circuits.iter().take(3).map(|inner| inner.len()).product::<usize>();
    println!("circuits: {:?}", circuits);
    println!("{}", product);

    // part 2
    println!("{}", junctions[distances[999].0.0].x * junctions[distances[999].0.1].x);

    // wait for user input
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
  }

  Ok(())
}