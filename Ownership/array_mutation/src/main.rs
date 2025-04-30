

#[allow(dead_code)]
fn array_mut_ownership(array: [u32; 5], operation: char, other_member: u32) -> [u32; 5] {
    let mut new_array: [u32; 5] = [0; 5];
    
    for (i,val) in array.iter().enumerate() {
        match operation { 
            '+' => new_array[i] = val + other_member,
            '-' => new_array[i] = val - other_member,
            '*' => new_array[i] = val * other_member,
            '/' => new_array[i] = val / other_member,
            _ => {} 
        }
    }
    new_array
}

#[allow(dead_code)]
fn array_mut_mut(array: &mut [u32], operation: char, other_member: u32) {
    for val in array.iter_mut() {
        match operation { 
            '+' => *val += other_member,
            '-' => *val -= other_member,
            '*' => *val *= other_member,
            '/' => *val /= other_member,
            _ => {} 
        }
    }
}

fn main() {}

#[cfg(test)]
mod array_mutation_test {

    const OWNERSHIP_TEST_ARRAY: [u32; 5] = [1, 2, 3, 4, 5];

    #[test]
    fn test_ownership_mutation() {
        assert_eq!(super::array_mut_ownership(OWNERSHIP_TEST_ARRAY, '+', 1), [2, 3, 4, 5, 6]);
        assert_eq!(super::array_mut_ownership(OWNERSHIP_TEST_ARRAY, '-', 1), [0, 1, 2, 3, 4]);
        assert_eq!(super::array_mut_ownership(OWNERSHIP_TEST_ARRAY, '*', 2), [2, 4, 6, 8, 10]);
        assert_eq!(super::array_mut_ownership(OWNERSHIP_TEST_ARRAY, '/', 2), [0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_mut_ref_mutation() {
        let mut array = OWNERSHIP_TEST_ARRAY.clone();

        super::array_mut_mut(&mut array, '+', 1);

        assert_eq!(array, [2, 3, 4, 5, 6]);

        let mut array = OWNERSHIP_TEST_ARRAY.clone();

        super::array_mut_mut(&mut array, '-', 1);

        assert_eq!(array, [0, 1, 2, 3, 4]);

        let mut array = OWNERSHIP_TEST_ARRAY.clone();

        super::array_mut_mut(&mut array, '*', 2);

        assert_eq!(array, [2, 4, 6, 8, 10]);

        let mut array = OWNERSHIP_TEST_ARRAY.clone();

        super::array_mut_mut(&mut array, '/', 2);

        assert_eq!(array, [0, 1, 1, 2, 2]);
    }

}