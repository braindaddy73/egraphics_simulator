use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hallo.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hallo.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem beim Erstellen der Datei: {:?}", e),
            },
            other_error => {
                panic!("Problem beim Öffnen der Datei: {:?}", other_error)
            }
        },
    };
}
