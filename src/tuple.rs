// Tuples group together values of different type
// Max of 12 elements

pub fn run() {
  let person: (&str, &str, i8) = ("Eder", "São Paulo", 19);

  println!(
    "\n{} is from {} and is {} years old",
    person.0, person.1, person.2
  );
}
