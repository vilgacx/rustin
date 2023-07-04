use rustyline;
use std::{
    fs::{create_dir, OpenOptions},
    io::{prelude::*,Seek,SeekFrom},
    path::Path,
    process::Command,
};

fn main() {
    if !Path::new("./dist").exists() {
        create_dir("./dist").expect("Error");
    }
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open("./dist/program.rs")
        .expect("Error");
    let mut rl = rustyline::DefaultEditor::new().expect("Error");
    let mut prompt = "> ";
    loop {
        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                if line == "exit" {
                    break;
                } else if line.chars().last().unwrap() == '{' {
                    prompt = "... ";
                } else if line.chars().last().unwrap() == '}' {
                    prompt = "> ";
                } else {
                    let code = format!("fn main(){{ {} }}", line);
                    file.seek(SeekFrom::Start(0)).unwrap(); 
                    file.write_all(code.as_bytes()).unwrap();
                    let fmt_out = Command::new("rustfmt")
                        .arg("dist/program.rs")
                        .output()
                        .expect("Error");
                    let c_out = Command::new("rustc")
                        .args(["dist/program.rs", "-o", "dist/program"])
                        .output()
                        .expect("Error");
                    let ex_out = Command::new("./dist/program").output().expect("Error");
                    println!(
                        "{}\n{}\n{}",
                        String::from_utf8_lossy(&fmt_out.stdout),
                        String::from_utf8_lossy(&c_out.stdout),
                        String::from_utf8_lossy(&ex_out.stdout)
                    );
                }
            }
            Err(_) => (),
        };
    }
}
