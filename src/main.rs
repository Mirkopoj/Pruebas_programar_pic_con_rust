use std::process::{Command, Stdio};
use std::io::{Write, Read};
use std::str;

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
        .expect("Falló pickle blank");

    clear.stdin.take().expect("No se abrió el stdin").write(b"y").expect("No se escribió");
    let mut respuesta: [u8;50] = [0;50];
    clear.stdout.expect("No se abrió el stdout").read(&mut respuesta).expect("Falló leer stdout");
    let clrstr = str::from_utf8(&respuesta).expect("No se puede convertir program");

    println!("blank");
    if clrstr.contains("pic16_read_config_memory"){
        println!("CACA");
    } else {
        println!("SIUUUUU");
    }

    let program = p16
        .arg("program")
        .arg("/home/dietpi/TestCode.hex")
        .output()
        .expect("Falló pickle program");

    let prgstr = String::from_utf8(program.stdout).expect("No se puede convertir program");

    println!("program");
    print!("{}",prgstr);
    if prgstr.contains("pic16_read_config_memory"){
        println!("CACA");
    } else {
        println!("SIUUUUU");
    }

    let verify = p16
        .arg("select")
        .output()
        .expect("Falló pickle verify");

    let verstr = String::from_utf8(verify.stdout).expect("No se puede convertir verify");

    println!("verify");
    print!("{}",verstr);
    if verstr.contains("Fail: 0"){
        println!("SIUUUUU");
    } else {
        println!("CACA");
    }
}
