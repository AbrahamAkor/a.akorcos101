use std::fs::File;
use std::io::Write;

struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}

fn main() {
    let students = vec![
        Student {
            name: "Oluchi Mordi".to_string(),
            matric_number: "ACC10211111".to_string(),
            department: "Accounting".to_string(),
            level: 300,
        },
        Student {
            name: "Adams Aliyu".to_string(),
            matric_number: "ECO10110101".to_string(),
            department: "Economics".to_string(),
            level: 100,
        },
        Student {
            name: "Shania Bolade".to_string(),
            matric_number: "CSC1032828".to_string(),
            department: "Computer".to_string(),
            level: 200,
        },
        Student {
            name: "Adekunle Gold".to_string(),
            matric_number: "EEE11020202".to_string(),
            department: "Electrical".to_string(),
            level: 200,
        },
        Student {
            name: "Blanca Edemoh".to_string(),
            matric_number: "MEE10200201".to_string(),
            department: "Mechanical".to_string(),
            level: 100,
        },
    ];

    println!("PAU SMIS");
    println!("{:<20} {:<15} {:<15} {:<5}", "Student Name", "Matric. Number", "Department", "Level");

    for student in &students {
        println!(
            "{:<20} {:<15} {:<15} {:<5}",
            student.name, student.matric_number, student.department, student.level
        );
    }


    let mut file = File::create("students.txt").unwrap();
    writeln!(file, "PAU SMIS").unwrap();
    writeln!(file, "{:<20} {:<15} {:<15} {:<5}", "Student Name", "Matric. Number", "Department", "Level").unwrap();
    
    for student in &students {
        writeln!(file, "{:<20} {:<15} {:<15} {:<5}", student.name, student.matric_number, student.department, student.level).unwrap();
    }

    println!("Student details have been saved to students.txt.");
}
