pub fn run() {
    greeting("Hello", "Eder");

    // Bind function values to variables
    let get_sum = add(5, 10);

    println!("The sum is: {}", get_sum);


    // Closure
    let num3: i32 = 1;

    let add_nums = |num1: i32, num2: i32| num1 + num2 + num3;

    println!("(Closure) The sum of 3 + 9 + 1 is {}", add_nums(3, 9));
}

fn greeting(greet: &str, name: &str) {
    println!("{}, {}! Nice to meet you!", greet, name); 
}

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
