

fn bubble_sort<T: Ord>(v: &mut Vec<T>) {
    let len = v.len();
    if len < 2 {
        return;
    }
    
    for i in 0..len {
        for j in 0..len - i - 1 {
            if v[j + 1] < v[j] {
                v.swap(j, j + 1);
            }
        }
    }
}

fn selection_sort<T: Ord>(v: &mut Vec<T>) {
    let len = v.len();
    if len < 2 {
        return;
    }
    
    for i in 0..len {
        let mut min = i;
        for j in i+1..len {
            if v[j] < v[min] {
                min = j;
            }
        }
        v.swap(i, min);
    }
}


fn main() {
    todo!()
}

mod tests {
    use crate::{bubble_sort, selection_sort};

    #[test]
    fn test_bubble_sort() {
        let mut bs1 = vec![10, 5, 8, 9, 7, 4, 3, 2, 1];
        let bs1s = [1, 2, 3, 4, 5, 7, 8, 9, 10];
        
        bubble_sort(&mut bs1);
        
        assert!(bs1s.iter().zip(bs1.iter()).all(|(a, b)| *a == *b))
    }
    
    #[test]
    fn test_selection_sort() {
        let mut bs1 = vec![10, 5, 8, 9, 7, 4, 3, 2, 1];
        let bs1s = [1, 2, 3, 4, 5, 7, 8, 9, 10];

        selection_sort(&mut bs1);

        assert!(bs1s.iter().zip(bs1.iter()).all(|(a, b)| *a == *b))
    }
}
