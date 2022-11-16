use std::process::Command;

fn main() {
    let id = Command::new("p16")
        .arg("id")
        .output()
        .expect("Falló pickle id");

    print!("{:?}", id.stdout); 
}
