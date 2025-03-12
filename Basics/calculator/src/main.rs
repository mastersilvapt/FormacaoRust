use std::io::{stdin, stdout, Write};
use regex::Regex;

//todo!("Implementar a calculadora que lê do stdin e faz as operações básicas +-*/")

fn calculator(input: &str) -> Option<i32> {
    let re = Regex::new(r"(-?\d+|\+|-|/|\*)").unwrap();
    let tokens = re.find_iter(&input).map(|x| x.as_str()).collect::<Vec<&str>>();

    if tokens.len() != 3 {
        return None;
    }
    let num1 = tokens[0].parse::<i32>().unwrap();
    let operator = tokens[1];
    let num2 = tokens[2].parse::<i32>().unwrap();

    match operator {
        "+" => Some(num1 + num2),
        "-" => Some(num1 - num2),
        "*" => Some(num1 * num2),
        "/" => Some(num1 / num2),
        _ => None,
    }
}

fn main() {
    let mut input = String::new();

    loop {
        print!("> ");
        stdout().flush().unwrap();

        input.clear();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().replace(" ", "");
        
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        match calculator(&input) {
            Some(num) => println!("R> {input} = {num}"),
            None => eprintln!("E> Please input a valid operation!"),
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        assert_eq!(super::calculator("2 + 2"), Some(4));
        assert_eq!(super::calculator("2 - 2"), Some(0));
        assert_eq!(super::calculator("2 * 2"), Some(4));
        assert_eq!(super::calculator("2 / 2"), Some(1));
    }
    
    #[test]
    fn negative_numbers(){
        assert_eq!(super::calculator("-2 + 2"), Some(0));
        assert_eq!(super::calculator("-2 - 2"), Some(-4));
        assert_eq!(super::calculator("-2 * 2"), Some(-4));
        assert_eq!(super::calculator("-2 / 2"), Some(-1));
        
        assert_eq!(super::calculator("2 + -2"), Some(0));
        assert_eq!(super::calculator("2 - -2"), Some(4));
        assert_eq!(super::calculator("2 * -2"), Some(-4));
        assert_eq!(super::calculator("2 / -2"), Some(-1));
        
        assert_eq!(super::calculator("-2 + -2"), Some(-4));
        assert_eq!(super::calculator("-2 - -2"), Some(0));
        assert_eq!(super::calculator("-2 * -2"), Some(4));
        assert_eq!(super::calculator("-2 / -2"), Some(1));
    }
    
    #[test]
    fn malformed_input() {
        assert_eq!(super::calculator("1b5"), None);
        assert_eq!(super::calculator("-1b5"), None);
        assert_eq!(super::calculator("-2147483648"), None);
        assert_eq!(super::calculator("-1e5"), None);
        assert_eq!(super::calculator("+inf"), None);
        assert_eq!(super::calculator("-inf"), None);
        assert_eq!(super::calculator("+123"), None);
        assert_eq!(super::calculator("-123"), None);
        assert_eq!(super::calculator("+123k"), None);
        assert_eq!(super::calculator("54-b"), None);
    }
}