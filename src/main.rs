use std::io;

fn main() {
    println!("Enter n:");

    let mut n = String::new();

    io::stdin().read_line(&mut n)
        .expect("Unable to read from input");

    let n: i32 = match n.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    if n % 5 == 0 && n % 3 == 0 {
        println!("FizzBuzz");
    }
    else if n % 5 == 0 {
        println!("Fizz");
    }
    else if n % 3 == 0 {
        println!("Buzz");
    }
}
