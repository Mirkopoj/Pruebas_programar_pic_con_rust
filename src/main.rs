use std::process::Command;

fn main() {
    let id = Command::new("p16")
        .arg("id")
        .output()
        .expect("Falló pickle id");

    let idstr = String::from_utf8(id.stdout).expect("No se puede convertir");

    print!("{}",idstr);

}
