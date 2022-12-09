use std::fs;

struct Elf {
  cal_sum: i32,
}

const FILE_PATH: &str = "src/input.txt";

fn main() {
  let file_contents = fs::read_to_string(FILE_PATH).unwrap();

  let mut elves: Vec<Elf> = vec![];
  let mut cur_sum = 0;
  let mut three_highest = 0;

  // Split up file into elves and their loads
  for l in file_contents.lines() {
    if l.is_empty() {
      elves.push(Elf { cal_sum: cur_sum });
      cur_sum = 0;
    } else {
      cur_sum += l.parse::<i32>().unwrap();
    }
  }

  // Sort array descending
  elves.sort_by(|a, b| b.cal_sum.cmp(&a.cal_sum));

  // Answer 1
  println!("{}", elves[0].cal_sum);

  // Answer 2
  for e in elves.iter().take(3) {
    three_highest += e.cal_sum;
  }
  println!("{}", three_highest)
}
