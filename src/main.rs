use rustyline;

fn main() {
    let mut rl = rustyline::DefaultEditor::new().expect("Error");
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                if line == "exit" { break }
            },
            Err(_) => (),
        };
    }
}
