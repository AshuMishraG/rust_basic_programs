// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * Implement Debug print functionality using `derive`
#[derive(Debug)]
struct Adult {
    //   * The structure must contain the person's name and age
    age: u8,
    name: String,
}
impl Adult {
    fn new (age: u8, name: &str) -> Result<Self, &str> {
        //   * The Ok variant should contain the initialized structure, but only
        //     if the person is aged 21 or older
        if age >= 21 {
            Ok(Self {
                age,
                name: name.to_string(),
            })
        } else {
            Err("Age must be at least 21 year")
        }
    }
}

// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
fn main() {
    // * Instantiate two `Adult` structures:
    //   * One should be aged under 21
    let child = Adult::new(15, "Anita");
    //   * One should be 21 or over
    let adult = Adult::new(21, "Sanjay");
    // * Use `match` to print out a message for each `Adult`:
    match child {
        //   * For the Ok variant, print any message you want
        Ok(child) => println!("{} is {} years old", child.name, child.age),
        //   * For the Err variant, print out the error message
        Err(e) => println!("{e}"),
    }

    match adult {
        //   * For the Ok variant, print any message you want
        Ok(a) => println!("{} is {} years old", a.name, a.age),
        //   * For the Err variant, print out the error message
        Err(e) => println!("{e}"),
    }
}