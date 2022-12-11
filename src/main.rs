use std::fs;

const FILE_PATH: &str = "src/input.txt";

fn main() {
  let file_contents = fs::read_to_string(FILE_PATH).unwrap();
  let mut sum = 0;
  let mut count = 0;
  let mut cur_lines: Vec<&str> = vec![];
  let mut cur_char: char = ' ';

  for l in file_contents.lines() {
    if count < 3 {
      cur_lines.push(l);
      count += 1;
      continue;
    }

    for c in cur_lines.first().unwrap().chars() {
      if cur_lines.get(1).unwrap().contains(c) && cur_lines.get(2).unwrap().contains(c) && c != cur_char {
        cur_char = c;
        if c.is_uppercase() {
          sum += 26;
        }

        let c = c.to_ascii_lowercase();

        sum += c as u16 - 96;
      }
    }

    cur_lines = vec![l];
    count = 1;
    cur_char = ' ';
  }

  println!("\n====Answer====\n{}", sum);
}
