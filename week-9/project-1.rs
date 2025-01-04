use std::io::Read;
use std::io::Write;
use std::fs::File;
fn main() {

    let mut file = File::create("drinks.txt").unwrap();
    writeln!(file, "Lager:").unwrap();

    writeln!(file, "33 Export").unwrap();
    writeln!(file, "Desperados").unwrap();
    writeln!(file, "Goldberg").unwrap();
    writeln!(file, "Gulder").unwrap();
    writeln!(file, "Heineken").unwrap();
    writeln!(file, "Star").unwrap();
    writeln!(file).unwrap(); 
    writeln!(file, "Stout:").unwrap();

    writeln!(file, "Legend").unwrap();
writeln!(file, "Turbo King").unwrap();
    writeln!(file, "Williams").unwrap();
    writeln!(file).unwrap(); 
    writeln!(file, "Non-Alcoholic:").unwrap();

    writeln!(file, "Maltina").unwrap();
    writeln!(file, "Amstel Malta").unwrap();
    writeln!(file, "Malta Gold").unwrap();
    writeln!(file, "Fayrouz").unwrap();
    let mut file = File::open("drinks.txt").unwrap();

    let mut read_contents = String::new();
    file.read_to_string(&mut read_contents).unwrap();
println!("{}", read_contents);

}
