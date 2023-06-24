use rustyline;
use std::fs::File;

fn main() {
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
                    //command write

                }
            },
            Err(_) => (),
        };
    }
}
