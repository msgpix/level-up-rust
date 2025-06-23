fn median(mut values: Vec<f32>) -> Option<f32> {
    let length = values.len();

    // A half - represents the midpoint index, though a nomenclature aligned with "entirety" bellow
    let moiety = length / 2;
    match length {
        0 => None,
        entirety => {
            values.sort_by(|former, latter| former.partial_cmp(latter).unwrap());

            // mes[i](o)- for "middle" and -axe for "value" so literally "the middle value".
            let mesiaxe = match entirety % 2 {
                0 => (values[moiety - 1] + values[moiety]) / 2.0,
                #[allow(unused_variables)] // rather than inconsiderately bend to the underscore convention
                otherwise => values[moiety],
            };

            Some(mesiaxe)
        }
    }
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty·list() {
    let input = vec![];
    let expected·output = None;
    let actual·output = median(input);
    assert_eq!(actual·output, expected·output);
}

#[test]
fn sorted·list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected·output = Some(4.0);
    let actual·output = median(input);
    assert_eq!(actual·output, expected·output);
}

#[test]
fn even·length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected·output = Some(4.0);
    let actual·output = median(input);
    assert_eq!(actual·output, expected·output);
}

#[test]
fn unsorted·list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected·output = Some(2.0);
    let actual·output = median(input);
    assert_eq!(actual·output, expected·output);
}
