fn main() {
    let name1 = "Abraham Akor";
    println!("My name is {}",name1);
    let name2 = name1.replace("Abraham","Sunday");
    println!("You can also call me: {}", name2);
    let faculty = "Faculty of Science and Technology";

    let school = faculty.replace("Faculty", "School");
    println!("I am a student of the {}",school);

}