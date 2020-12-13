// Enums are types whitch have a few definite values

enum Movement {
  // Variants
  Up,
  Down,
  Left,
  Right,
}

fn move_avatar(m: Movement) {
  // Perform action depending on info
  match m {
    Movement::Up => println!("Player moving up"),
    Movement::Down => println!("Player moving down"),
    Movement::Left => println!("Player moving left"),
    Movement::Right => println!("Player moving right"),
  }
}

pub fn run() {
  let avatar1 = Movement::Up;
  let avatar2 = Movement::Down;
  let avatar3 = Movement::Left;
  let avatar4 = Movement::Right;

  move_avatar(avatar1);
  move_avatar(avatar2);
  move_avatar(avatar3);
  move_avatar(avatar4);
}
