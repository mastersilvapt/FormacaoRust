#[warn(dead_code)]
fn fibonacci(n: u32) -> u32 {
    //todo!("Implementar fibonacci aqui")
    if n == 0 || n == 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

#[cfg(test)]
mod fibonacci_test {

    #[test]
    fn test_fibonacci() {
        assert_eq!(super::fibonacci(0), 0);
        assert_eq!(super::fibonacci(1), 1);
        assert_eq!(super::fibonacci(2), 1);
        assert_eq!(super::fibonacci(3), 2);
        assert_eq!(super::fibonacci(4), 3);
        assert_eq!(super::fibonacci(5), 5);
        assert_eq!(super::fibonacci(6), 8);
        assert_eq!(super::fibonacci(7), 13);
        assert_eq!(super::fibonacci(8), 21);
        assert_eq!(super::fibonacci(9), 34);
        assert_eq!(super::fibonacci(10), 55);
    }

}

fn main() {}