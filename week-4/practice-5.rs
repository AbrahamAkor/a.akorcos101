use std:: io;
fn main() {
    let mut input = String::new();

    println!("170.0cm: ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height:f32 = input.trim().parse().expect("NOt a valid number");

    if height > 150.0 && height < 170.0 {
        println!("Your are of average height person")
    }
    else if height > 170.0 && height < 195.0
    {
        println!("You are tall");
    }
    else if height <150.0 && height > 100.0

    {
        println!("You are dwarf")
    }
    else {
        println!("Abnormal height");

    }
}