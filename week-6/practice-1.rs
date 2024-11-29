fn main() {
    let name = "Abraham Akor";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str = "Kn %2 Lekki-Epe Expressway, Ibeju-lekki, Lagos";
    println!("Name: {}", name);
    println!("University: {}, \naddress: {}",uni, addr);

    let department:&'static str = "Computer Science";
    let school:&'static str = "School of Science and Technology";
    println!("Department: {}, \nSchool: ()",department);

}