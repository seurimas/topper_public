fn combinations_util<A: Clone>(
    complete_combinations: &mut Vec<Vec<A>>,
    items: &[A],
    mut current: Vec<A>,
    len: usize,
) {
    if items.len() == len {
        current.extend_from_slice(items);
        complete_combinations.push(current);
    } else if len == 0 {
        complete_combinations.push(current);
    } else if items.len() > len {
        combinations_util(complete_combinations, &items[1..], current.clone(), len);
        let mut added = current.clone();
        added.push(items[0].clone());
        combinations_util(complete_combinations, &items[1..], added, len - 1);
    } else {
        panic!("Impossible!");
    }
}

pub fn combinations<A: Clone>(items: &[A], len: usize) -> Vec<Vec<A>> {
    let mut combination_vec = Vec::new();
    combinations_util(&mut combination_vec, items, Vec::new(), len);
    combination_vec
}

#[cfg(test)]
mod combinatorics_test {
    use super::*;

    #[test]
    fn test_null_combination() {
        let nulls = combinations(&[1, 2, 3], 0);
        assert_eq!(nulls.len(), 1);
    }

    #[test]
    fn test_single_combination() {
        let singles = combinations(&[1, 2, 3], 1);
        assert_eq!(singles, vec![vec![3], vec![2], vec![1]]);
    }

    #[test]
    fn test_double_combination() {
        let doubles = combinations(&[1, 2, 3], 2);
        assert_eq!(doubles, vec![vec![2, 3], vec![1, 3], vec![1, 2]]);
    }

    #[test]
    fn test_full_combination() {
        let full = combinations(&[1, 2, 3], 3);
        assert_eq!(full, vec![vec![1, 2, 3]]);
    }
}
