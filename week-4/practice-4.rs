use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    
    println!("Enter first adge of triangle: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Enter first adge of triangle:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:i32 = input2.trim().parse().expect("NOt a valid number");
    if age > 18 {
        println!("Welcome to the party {}", input1)
    } else {
        println!("Oops< you are not pf age to enter the party {}", input1);
    }
}