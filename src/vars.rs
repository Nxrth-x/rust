// Vairables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
  let name = "Eder";
  let age = 19;
  println!("My name is {} and I'm {}!", name, age);

  // Can't reasign values to a variable
  // This will break the code
  // age = 20;
  // To mutate variables we use let mut
  let mut my_age = age;
  my_age += 1;
  println!("I'll be {} years old next year. 👴", my_age);

  // Defining constants
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assigning multiple variables at once
  let (my_name, my_surname) = ("Eder", "Lima");
  println!("My name is: {} {}", my_name, my_surname);
}
