/*

  Primitive types
  Integers: u8, i8, u16, i16, [...], i128
  Floats: f32, f64
  Boolean (bool)
  Characters (char)
  Tuples
  Arrays

*/

// Rust is a statically typed language, so it MUST know the types of all variables at compile time
// Though the compile can infer the type of the variable we want to use

pub fn run() {
  // Default is "i32"
  let x = 10;

  // Default is "f64"
  let y = 1.5;

  // Add explicit type
  let z: f64 = 3245234523452345.2345;

  // Find max size
  println!("Max i32: {}", std::i32::MAX);
  println!("Max f64: {}", std::i64::MAX);

  // Boolean
  let is_active: bool = true;

  // Boolean from an expression
  let is_greater: bool = 1 > 2;

  // Chars
  let a1: char = 'a';
  let cowboy: char = '\u{1F920}'; // Unicode character

  println!("{:?}", (x, y, z, is_active, is_greater, a1, cowboy));
}
