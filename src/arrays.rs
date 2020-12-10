// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
  // Immutable array
  let numbers: [i32; 5] = [1, 2, 3, 4, 5];

  println!("\nArray: {:?}", numbers);

  // Get single value
  println!("Single value: {}", numbers[0]);

  // Mutable array
  let mut mutable_numbers: [i32; 5] = [5, 4, 3, 2, 1];

  mutable_numbers[0] = 4;
  println!("{:?}", mutable_numbers);

  // Get array length
  println!("Array length: {}", mutable_numbers.len());

  // Arrays are stack allocated
  println!("Array occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);

  println!("Slice: {:?}", &numbers[1..5]);
}
