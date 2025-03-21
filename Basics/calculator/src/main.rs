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

/// Desafio da calculadora
/// Pode ser implementado com recurso à analise de apenas
/// uma string ou com cada elemento separado na sua propria string.
/// Podem ver exemplos de como vai ser utilizada a função nos testes disponíveis.
///
/// Devem apenas implementar uma das funções.
///
/// Podem comentar a função que não vão implementar para não haver problemas de compilação, incluindo os testes
/// para a mesma.

fn main() {
    todo!("Implementar a leitura do stdin")
}


fn calculator_str(string: &str) -> i32 {
    todo!("Implementar a calculadora que de uma string calcule o resultado")
}

fn calculator_str_list(string: &[&str]) -> i32 {
    todo!("Implementar a calculadora que de uma string ou de uma lista de strings calcule o resultado")
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
        assert_eq!(super::calculator_str_list(&vec!["3", "-", "2"]), 0);
        assert_eq!(super::calculator_str_list(&vec!["6", "/", "3"]), 2);
    }

}