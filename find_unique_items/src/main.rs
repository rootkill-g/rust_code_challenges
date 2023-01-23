fn youneek(mut a: Vec<i32>) -> Vec<i32> {
    // No Duplicates from the vector a to be put into the res vector
    a.sort();
    a.dedup();
    a
}

// Advanced: 1
fn generic_unique<T: Ord>(mut a: Vec<T>) -> Vec<T> {
    a.sort();
    a.dedup();
    a
}

fn main() {
    let list_1 = vec![2, 1, 1];

    let answer = youneek(list_1);

    println!("list_1: {:?}", answer);

    let list_2 = vec![2, 3, 3, 1, 8, 7, 5, 1, 9, 4, 7, 9];

    println!("list_2: {:?}", generic_unique(list_2));
}

#[test]
fn empty_list() {
    let list = vec![];

    assert_eq!(youneek(list), vec![]);
}

#[test]
fn test_generic() {
    let list = vec![1, 2, 6, 3, 1, 6, 6, 2, 7, 3, 9, 4];
    assert_eq!(generic_unique(list), vec![1, 2, 3, 4, 6, 7, 9]);
}
