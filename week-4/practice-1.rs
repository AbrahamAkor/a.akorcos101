use std::io;

fn main() {
    println!("Student Information Management System");

    println!("\nPlease enter your name.");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");
    let name = name.trim();
    println!("Your name is: {}", name);


    println!("\nEnter your age.");
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read input");
    
    let age: i32 = match age.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, age must be an integer.");
            return;
        }
    };

    
    println!("Your age is: {}", age);
}
