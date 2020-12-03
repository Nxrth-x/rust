// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use whe you need to modify or own string data

pub fn run(){
	let mut hello = String::from("Hello");
	
	println!("Original string:\n{}\n", hello);

	// Push a string
	hello.push_str(", world! ");
	println!("Mutated string (added string):\n{}\n", hello);

	// Add a character to the string
	hello.push('\u{1F920}');
	println!("Mutated string (added character):\n{}\n", hello);

	// Get string length
	println!("Length:\n{}\n", hello.len());

	// Capacity (in bytes)
	println!("Capacity: {}\n", hello.capacity());

	// Check is string is empty
	println!("String is empty?\n{}\n", hello.is_empty());

	// Contains substring
	println!("Contains 'world'?\n{}\n", hello.contains("world"));

	// Replace
	println!("Replace:\n{}\n", hello.replace(" \u{1F920}", " \u{1F30E}"));

	// Loop through string by whitespace
	println!("Looping through a string:");
	for word in hello.split_whitespace() {
		println!("{}", word);
	}

	// Create string with fixed capacity
	let mut s = String::with_capacity(10);
	s.push('a');
	s.push('b');

	println!("\nString with fixed capacity: {}", s);

	// Assertion testing
	assert_eq!(2, s.len());
	assert_eq!(10, s.capacity());
}