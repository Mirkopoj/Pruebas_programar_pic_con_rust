use std::process::Command;

fn main() {
    let id = Command::new("p16")
        .arg("id")
        .output()
        .expect("Falló pickle id");

    let idstr = String::from_utf8(id.stdout).expect("No se puede convertir");

    if idstr.contains("pic16_read_config_memory"){
        println!("CACA");
    } else {
        println!("SIUUUUU");
    }

}
