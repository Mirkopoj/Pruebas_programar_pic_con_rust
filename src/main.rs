use std::process::{Command, Stdio};
use std::io::{Write, Read};

fn main() {
    let mut p16 = Command::new("p16");
    let id = p16
        .arg("id")
        .output()
        .expect("Falló pickle id");

    let idstr = String::from_utf8(id.stdout).expect("No se puede convertir id");

    println!("id");
    if idstr.contains("pic16_read_config_memory"){
        println!("CACA");
    } else {
        println!("SIUUUUU");
    }

    let mut clear = p16
        .arg("blank")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Falló pickle id");

    clear.stdin.take().expect("No se abrió el stdin").write(b"y").expect("No se escribió");
    let respuesta = String::new();
    clear.stdout.expect("No se abrió el stdout").read(&respuesta);
    //let prgstr = String::from_utf8(respuesta).expect("No se puede convertir program");

    println!("blank");
    if respuesta.contains("pic16_read_config_memory"){
        println!("CACA");
    } else {
        println!("SIUUUUU");
    }

    let program = p16
        .arg("program")
        .arg("/home/dietpi/TestCode.hex")
        .output()
        .expect("Falló pickle id");

    let prgstr = String::from_utf8(program.stdout).expect("No se puede convertir program");

    println!("program");
    if prgstr.contains("pic16_read_config_memory"){
        println!("CACA");
    } else {
        println!("SIUUUUU");
    }

    let verify = p16
        .arg("verify")
        .arg("/home/dietpi/TestCode.hex")
        .output()
        .expect("Falló pickle id");

    let verstr = String::from_utf8(program.stdout).expect("No se puede convertir program");

    println!("verify");
    if prgstr.contains("Fail: 0"){
        println!("SIUUUUU");
    } else {
        println!("CACA");
    }
}
