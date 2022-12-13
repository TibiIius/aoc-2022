use std::fs;

const FILE_PATH: &str = "src/input.txt";

fn main() {
  let file_contents = fs::read_to_string(FILE_PATH).unwrap();
  let mut sum = 0;
  let mut chars_vec: Vec<char> = vec![];

  for l in file_contents.lines() {
    while chars_vec.len() < 4 {
      let mut iter = l.chars();
      if sum > 0 {
        for _ in 0..sum {
          iter.next();
        }
      }
      chars_vec = iter.take(4).collect();
      println!("{:?}", chars_vec);
      chars_vec.sort_unstable();
      chars_vec.dedup();
      sum += 4;
    }
  }

  println!("\n====Answer====\n{}", sum);
}
