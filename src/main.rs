fn median(mut a: Vec<f32>) -> Option<f32> {
    if a.is_empty() {
        return None;
    }

    a.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let len = a.len();

    if len % 2 == 0 {
        // Even length: average of the two middle elements
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        Some((a[mid_left] + a[mid_right]) / 2.0)
    } else {
        // Odd length: middle element
        Some(a[len / 2])
    }
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}


#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
