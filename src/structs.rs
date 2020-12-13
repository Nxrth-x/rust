// Structs - Used to create data types

// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
struct ColorStruct(u8, u8, u8);

// Struct with functions associated with it
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Constructs new person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string()
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}



pub fn run() {
    let mut c = Color {
        red: 255,
        green: 30,
        blue: 30,
    };

    c.green = 40;
    c.blue = 40;

    println!("Color: {}, {}, {}.", c.red, c.green, c.blue);

    let mut c_tuple = ColorStruct(255, 20, 30);

    println!("Color (tuple struct): {}, {}, {}", c_tuple.0, c_tuple.1, c_tuple.2); 

    // Creating a new person
    let mut p = Person::new("Eder", "Lima");

    println!("Person: {} {}", p.first_name, p.last_name);

    // Using last name method
    println!("Person (full_name method): {}", p.full_name());

    // Changing last name
    p.set_last_name("Oliveira");
    println!("Person (set_last_name method): {}", p.full_name());


    // Persons name to tuple
    println!("Person (to tuple): {:?}", p.to_tuple());
}
