// ex4.rs
// Make me compile!

fn something() -> Result<i32, std::num::ParseIntError> {
    match "x3".parse() {Ok(x) => Ok(x * 4), err => return err}
}

fn main() {
    match something() {
        Ok(..) => println!("You win!"),
        Err(e) => println!("Oh no something went wrong: {}", e),
    }
}
