pub fn run() {
  // Print to console
  println!("Hello, from the print.rs file!");

  // Printing a variable
  println!("Number: {}", 1);

  // Printing muliple variables - Basic Formatting
  println!("{} is {} years old", "Eder", 19);

  // Positional arguments
  println!(
    "{0} is from {1} and {0} likes to {2}",
    "Eder", "São Paulo", "code"
  );

  // Named arguments
  println!(
    "{name} likes to play {activity}.",
    name = "John",
    activity = "soccer"
  );

  // Placeholder traits
  println!("Binary: {:b}\nHex: {:x}\nOctal: {:o}", 10, 10, 10);

  // Placeholder for debug traits
  println!("{:?}", (12, true, "Hello?"));

  // Basic math
  println!("10 + 10 = {}", 10 + 10);
}
