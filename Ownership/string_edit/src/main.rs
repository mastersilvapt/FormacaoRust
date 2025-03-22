use std::io::{stdin, stdout, Write};
use std::str::{FromStr};

enum Operation {
    Set(String),
    Clear,
    Remove(usize),
    Insert(usize, String),
    Uppercase(),
    Lowercase(),
    VigenereCipher(String),
    VigenereDecipher(String),
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split_whitespace();
        let op = splits.next().unwrap();
        match op {
            "Set" => {
                let string = &s[4..s.len()];
                Ok(Operation::Set(string.to_string()))
            }
            "Clear" => Ok(Operation::Clear),
            "Remove" => {
                let idx = splits
                    .next()
                    .ok_or("Command usage:\n Remove index")?
                    .parse::<usize>()
                    .map_err(|_| "Index must be an integer")?;
                Ok(Operation::Remove(idx))
            }
            "Insert" => {
                let idx = splits
                    .next()
                    .ok_or("Command usage:\n Insert index string")?
                    .parse::<usize>()
                    .map_err(|_| "Index must be an integer")?;

                let string = &s.splitn(3, " ").collect::<Vec<&str>>()[2];
                Ok(Operation::Insert(idx, string.to_string()))
            }
            "Uppercase" => Ok(Operation::Uppercase()),
            "Lowercase" => Ok(Operation::Lowercase()),
            "Vigenere" => {
                match splits.next().unwrap() { 
                    "cipher" => {
                        let key = &s[16..s.len()];
                        Ok(Operation::VigenereCipher(key.to_string()))
                    }
                    "decipher" => {
                        let key = &s[18..s.len()];
                        Ok(Operation::VigenereDecipher(key.to_string()))
                    }
                    _ => Err("Option only supports cipher and decipher".to_string()),
                }
            }
            _ => Err("Invalid operation".to_string()),
        }
    }
}

impl Operation {
    fn help() {
        println!("Commands available: ");
        println!("\t1 - Set string - Sets the string;");
        println!("\t2 - Clear - Clears the string;");
        println!("\t3 - Remove idx - Removes position 'idx' from string");
        println!("\t4 - Insert idx string - Inserts 'string' at 'idx'");
        println!("\t5 - Uppercase - Uppercases the string");
        println!("\t6 - Lowercase - Lowercases the string");
        println!("\t7 - Vigenere cipher string - Applies 'string' as Vigenere Cypher key to cypher text");
        println!("\t8 - Vigenere decipher string - Applies 'string' as Vigenere Cypher key to decipher text");
        println!("\t9 - Back");
        println!();
    }

    fn cipher(c: &u8, k: u8) -> char {
        if c.is_ascii_uppercase() {
            let b = 'A' as u8;
            (b + ((c-b + k-b) % 26)) as char
        }else if c.is_ascii_lowercase() {
            let b = 'a' as u8;
            (b + ((c-b + k-b) % 26)) as char
        }else {
            *c as char
        }
    }
    
    fn decipher(c: &u8, k: u8) -> char {
        if c.is_ascii_uppercase() {
            let b = 'A' as u8;
            let mut c = c-b;
            let mut k = k-b;
            if c < k {
                k = k-c;
                c = 26;
            }
            (b + ((c - k) % 26)) as char 
        }else if c.is_ascii_lowercase() {
            let b = 'a' as u8;
            let mut c = c-b;
            let mut k = k-b;
            if c < k {
                k = k-c;
                c = 26;
            }
            (b + ((c - k) % 26)) as char
        }else {
            *c as char
        }
    }
    fn apply(&self, input: &str) -> String {
        let mut input = input.to_string();
        match &self {
            Operation::Set(s) => {
                input = s.to_string();
            }
            Operation::Clear => {
                input.clear();
            }
            Operation::Remove(idx) => {
                input.remove(*idx);
            }
            Operation::Insert(idx, s) => {
                input.insert_str(*idx, &s.to_string());
            }
            Operation::Uppercase() => {
                input = input.to_string().to_uppercase();
            }
            Operation::Lowercase() => {
                input = input.to_string().to_lowercase();
            }
            Operation::VigenereCipher(key) => {
                let mut res = String::with_capacity(input.len());
                let key = key.as_bytes();
                let input_bytes = input.as_bytes();
                for (i, c) in input_bytes.iter().enumerate() {
                    res.push(Operation::cipher(c, key[i % key.len()]));
                }
                input = res.to_string()
            }
            Operation::VigenereDecipher(key) => {
                let mut res = String::with_capacity(input.len());
                let key = key.as_bytes();
                let input_bytes = input.as_bytes();
                for (i, c) in input_bytes.iter().enumerate() {
                    res.push(Operation::decipher(c, key[i % key.len()]));
                }
                input = res.to_string()
            }
        };
        println!("R> {input}");
        input
    }
}

fn main() {
    let mut string = String::new();
    loop {
        Operation::help();
        print!(">");
        stdout().flush().unwrap();
        let mut op = String::new();
        stdin().read_line(&mut op).expect("Failed to read line");
        let op = op.trim();
        if op.eq_ignore_ascii_case("Back") {
            break;
        }
        match op.parse::<Operation>() {
            Ok(op) => {
                string = op.apply(string.as_str());
            }
            Err(_) => {
                println!("E> Invalid operation");
            }
        }
    }
}
