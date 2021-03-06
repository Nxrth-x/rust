struct Cat {
    name: String,
    age: u8,
    furry: bool,
    lazy: bool,
}

impl Cat {
    fn new(name: &str, age: u8, furry: bool, lazy: bool) -> Cat {
        Cat {
            name: name.to_string(),
            age: age,
            furry: furry,
            lazy: lazy
        }
    }
    
    fn meow(&self) -> String {
        format!("{} says: meow! 😺", self.name)
    }
    
    fn is_lazy(&self) -> String {
        match self {
            Cat { name, lazy, .. } => format!("{} {} lazy... 🐈", name, if *lazy { "is" } else {"is not"})
        }
    }
    
    fn show_info(&self) -> String {
        match self {
            Cat { name, age, furry, lazy } => {
                format!("{} is {} years old, {} furry and {} lazy! 😸", *name, *age, if *furry { "is" } else { "is not" }, if *lazy { "is" } else { "is not" })
            }
        }
    }
}

fn main() {
    let mut cats: Vec<Cat> = vec![];
    cats.push(Cat::new("Safira", 2, true, false));
    cats.push(Cat::new("Tom", 2, false, true));
    
    println!("{}", cats[0].meow());
    println!("{}", cats[1].is_lazy());
    println!("{}", cats[0].show_info());
}
