use std::env;

pub fn run() {
  let args: Vec<String> = env::args().collect();
  let command: String = args[1].clone();

  if command == "hello" {
    let name: String = args[2].clone();

    println!("Hello, {name}!", name = name);
  }
}
