use std::fs;

const FILE_PATH: &str = "src/input.txt";

fn main() {
  let file_contents = fs::read_to_string(FILE_PATH).unwrap();
  let mut sum = 0;

  for l in file_contents.lines() {
    let l_vec: Vec<&str> = l.split(',').collect();
    let (part1, part2) = (l_vec.first().unwrap().to_owned(), l_vec.last().unwrap().to_owned());
    let part1_nums: Vec<&str> = part1.split('-').collect();
    let part2_nums: Vec<&str> = part2.split('-').collect();
    let (part1_lower, part1_higher) = (
      part1_nums.first().unwrap().parse::<i32>().unwrap(),
      part1_nums.last().unwrap().parse::<i32>().unwrap(),
    );
    let (part2_lower, part2_higher) = (
      part2_nums.first().unwrap().parse::<i32>().unwrap(),
      part2_nums.last().unwrap().parse::<i32>().unwrap(),
    );

    if (part1_lower <= part2_lower && part1_higher >= part2_higher)
      || (part2_lower <= part1_lower && part2_higher >= part1_higher)
    {
      sum += 1;
    }
  }

  println!("\n====Answer====\n{}", sum);
}
