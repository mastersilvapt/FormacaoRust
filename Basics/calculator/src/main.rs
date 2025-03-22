
/// Desafio da calculadora
/// Pode ser implementado com recurso à analise de apenas
/// uma string ou com cada elemento separado na sua propria string.
/// Podem ver exemplos de como vai ser utilizada a função nos testes disponíveis.
///
/// Devem apenas implementar uma das funções.
///
/// Podem comentar a função que não vão implementar para não haver problemas de compilação, incluindo os testes
/// para a mesma.

use std::io::{stdin, stdout, Write};
use regex::{Regex};

fn main() {
    // to do!("Implementar a leitura do stdin")
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

        println!("R> {input} = {}", calculator_str(&input))
    }
}


fn calculator_str(string: &str) -> i32 {
    let string = string.replace(" ", "");
    let re = Regex::new(r"(-?\d+)([+-/*])(-?\d+)").unwrap();
    let tokens = re.captures(&string);

    match tokens {
        Some(caps) => {

            let num1 = caps[1].parse::<i32>().unwrap();
            let op = &caps[2];
            let num2 = caps[3].parse::<i32>().unwrap();

            match op {
                "+" => num1 + num2,
                "-" => num1 - num2,
                "*" => num1 * num2,
                "/" => num1 / num2,
                _ => -1,
            }
        }
        None => -1,
    }
    //to do!("Implementar a calculadora que de uma string calcule o resultado")
}

fn calculator_str_list(string: &[&str]) -> i32 {
    //to do!("Implementar a calculadora que de uma string ou de uma lista de strings calcule o resultado")

    calculator_str(string.concat().as_str())
}

#[cfg(test)]
pub mod calculator_test {

    #[test]
    fn test_calculator_str() {
        assert_eq!(super::calculator_str("1 + 1"), 2);
        assert_eq!(super::calculator_str("2 * 2"), 4);
        assert_eq!(super::calculator_str("2 / 2"), 1);
        assert_eq!(super::calculator_str("2 - 2"), 0);
    }
    
    #[test]
    fn test_calculator_str_list() {
        assert_eq!(super::calculator_str_list(&vec!["2", "*", "3"]), 6);
        assert_eq!(super::calculator_str_list(&vec!["2", "+", "3"]), 5);
        assert_eq!(super::calculator_str_list(&vec!["3", "-", "2"]), 1);
        assert_eq!(super::calculator_str_list(&vec!["6", "/", "3"]), 2);
    }
}
