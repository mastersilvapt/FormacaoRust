use std::io::{stdin, stdout, Write};

pub fn read_valid_stdin<F, T>(prompt: &str, maplidator: F) -> T
where
    F: Fn(String) -> Result<T, &'static str>,
{
    loop {
        let mut input = String::new();
        print!("{}", prompt);
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        match maplidator(input) {
            Ok(value) => break value,
            Err(msg) => println!("{}", msg),
        }
    }
}
