fn main (){
let fullname = "Abraham Akor";
let department = "Software Enginerring";
let uni = "Pan Atlantic University";

let mut school = "School of Science".to_string();

school.push_str("and Technology");

println!("My name is: {}", fullname);
println!("The length my fullname is: {}", fullname);
println!("I am a student of {} Department", department);
println!("{}", school);
println!("{}", uni);
}