use std::fs;

const FILE_PATH: &str = "src/input.txt";

const ROCK: char = 'A';
const PAPER: char = 'B';
const SCISSORS: char = 'C';

const ROCK_ME: char = 'X';
const PAPER_ME: char = 'Y';
const SCISSORS_ME: char = 'Z';

fn main() {
  let file_contents = fs::read_to_string(FILE_PATH).unwrap();
  let mut my_score = 0;

  // Play each round (ugly but simple)
  for l in file_contents.lines() {
    // Split into enemy and own choice
    let enemy_choice = l.chars().nth(0).unwrap();
    let my_choice = l.chars().nth(2).unwrap();

    // add base score for the chosen type + see if we won or lost, or if it's a draw
    match my_choice {
      ROCK_ME => {
        match enemy_choice {
          // draw
          ROCK => my_score += 3,
          // won
          SCISSORS => my_score += 6,
          // lost, or anything else
          _ => {}
        }
        my_score += 1
      }
      PAPER_ME => {
        match enemy_choice {
          // draw
          PAPER => my_score += 3,
          // won
          ROCK => my_score += 6,
          // lost, or anything else
          _ => {}
        }
        my_score += 2
      }
      SCISSORS_ME => {
        match enemy_choice {
          // draw
          SCISSORS => my_score += 3,
          // won
          PAPER => my_score += 6,
          // lost, or anything else
          _ => {}
        }
        my_score += 3
      }
      _ => {}
    }
  }

  // Answer 1
  println!("\n========My Score========");
  println!("{}", my_score);
}
