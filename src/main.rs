use std::fs;

const FILE_PATH: &str = "src/input.txt";

const ROCK: char = 'A';
const PAPER: char = 'B';
const SCISSORS: char = 'C';

const LOOSE: char = 'X';
const DRAW: char = 'Y';
const WIN: char = 'Z';

fn main() {
  let file_contents = fs::read_to_string(FILE_PATH).unwrap();
  let mut my_score = 0;

  // Play each round (ugly but simple)
  for l in file_contents.lines() {
    // Split into enemy and own choice
    let enemy_choice = l.chars().nth(0).unwrap();
    let my_choice = l.chars().nth(2).unwrap();

    // add base score for the chosen type + see if we won or lost, or if it's a draw
    match enemy_choice {
      ROCK => match my_choice {
        LOOSE => my_score += 3,
        DRAW => my_score += 1 + 3,
        WIN => my_score += 2 + 6,
        _ => {}
      },
      PAPER => match my_choice {
        LOOSE => my_score += 1,
        DRAW => my_score += 2 + 3,
        WIN => my_score += 3 + 6,
        _ => {}
      },
      SCISSORS => match my_choice {
        LOOSE => my_score += 2,
        DRAW => my_score += 3 + 3,
        WIN => my_score += 1 + 6,
        _ => {}
      },
      _ => {}
    }
  }

  // Answer 1
  println!("\n========My Score========");
  println!("{}", my_score);
}
