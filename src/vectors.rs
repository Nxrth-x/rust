// Vectors are resizable arrays

use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1, 2, 3];
    
    // Re-asign value
    numbers[2] = 6;

    // Add on to vector
    numbers.push(5);
    numbers.push(10);

    println!("Vector {:?}", numbers);

    // Pop method (Remove the last item)
    numbers.pop();
    println!("Vector (after the pop method): {:?}", numbers);

    // Get single value from the vector
    println!("Single value: {}", numbers[0]);

    // Get vector length
    println!("Vector length: {}", numbers.len());

    // Vectors are stack allocated
    print!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..1];
    println!("\nSlice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate value
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Vector after being mutated: {:?}", numbers);
}
