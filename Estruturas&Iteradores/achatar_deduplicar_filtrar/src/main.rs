use std::collections::HashSet;

#[allow(dead_code)]
fn achatar_deduplicar_filtrar(v: Vec<Vec<u32>>) -> Vec<u32> {
    v.iter()
        .flatten()
        .collect::<HashSet<&u32>>()
        .into_iter()
        .filter(|x| **x % 2 == 0 || **x % 3 == 0)
        .copied()
        .collect::<Vec<u32>>()
}

fn main() {}

#[cfg(test)]
mod achatar_deduplicar_filtrar_test {
    use std::collections::HashSet;

    #[test]
    fn test_func() {

        let vec = vec![vec![1, 2, 3], vec![3, 4, 5], vec![5, 6, 7]];

        let result = super::achatar_deduplicar_filtrar(vec);

        assert!(result.iter().all(|x| x % 2 == 0 || x % 3 == 0));

        let mut seen = HashSet::new();

        assert!(result.iter().all(|x| seen.insert(x)));
    }

}