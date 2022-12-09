use std::fs;

const FILE_PATH: &str = "src/input.txt";

fn main() {
  let file_contents = fs::read_to_string(FILE_PATH).unwrap();
  let mut sum = 0;
  let mut cur_chars: Vec<char> = vec![];

  for l in file_contents.lines() {
    let mid: usize = l.len() / 2;
    let (s1, s2) = l.split_at(mid);

    for c in s2.chars() {
      if s1.contains(c) && !cur_chars.contains(&c) {
        cur_chars.push(c);
        // this way we only need to check the char once and can just add the offset to the result
        if c.is_uppercase() {
          sum += 26;
        }

        let c = c.to_ascii_lowercase();

        sum += c as u16 - 96;
      }
    }
    cur_chars = vec![];
  }

  println!("====Answer====\n{}", sum);
}
