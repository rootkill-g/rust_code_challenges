fn median(mut a: Vec<f64>) -> Option<f64> {
    // -- Empty List
    // -- Odd number of elements
    // -- Even number of elements

    if a.is_empty() {
        return None;
    }

    a.sort_by(|x, y| x.partial_cmp(y).unwrap());

    let n_elements = a.len();
    let middle = n_elements / 2;
    let median = if n_elements % 2 == 0 {
        // Even
        (a[middle - 1] + a[middle]) / 2.0
    } else {
        // Odd
        a[middle]
    };

   Some(median)
}

fn main() {
    // Odd number of elements
    let list = vec![1.0, 2.0, 3.0];
    assert_eq!(median(list), Some(2.0));
    // Even number of elements
    let list_2 = vec![1.0, 3.0, 5.0, 7.0];
    assert_eq!(median(list_2), Some(4.0));
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn odd_number_of_elements() {
    let list = vec![1.0, 2.0, 3.0];
    assert_eq!(median(list), Some(2.0))
}

#[test]
fn even_number_of_elements() {
    let list = vec![1.0, 3.0, 5.0, 7.0];
    assert_eq!(median(list), Some(4.0))
}
