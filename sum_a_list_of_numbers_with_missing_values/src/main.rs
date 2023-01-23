fn sum_with_missing(list: &Vec<Option<i32>>) -> i32 {
    let mut sum = 0;

    for val in list {
        match val {
            Some(n) => { sum += n },
            None => continue
        }
    }

    sum
}

fn main() {
    let list = vec![
        Some(1),
        None,
        Some(4),
        Some(7),
        None,
        Some(5),
    ];

    assert_eq!(sum_with_missing(&list), 17);

    println!("Sum of the list is : {}", sum_with_missing(&list));
}

#[test]
fn empty() {
    let nn = vec![];
    assert_eq!(sum_with_missing(&nn), 0);
}

#[test]
fn no_missing() {
    let n = vec![Some(1), Some(2), Some(3), Some(4)];
    assert_eq!(sum_with_missing(&n), 10);
}

#[test]
fn some_missing() {
    let n = vec![Some(1), None, Some(3), None, Some(5)];
    assert_eq!(sum_with_missing(&n), 9);
}
