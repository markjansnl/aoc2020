use aoc01::input;

fn main() {
    if let Some((a, b)) = find_sum_2020(&input::USER) {
        println!("{}", a * b);
    }
}

fn find_sum_2020(input: &[i32]) -> Option<(i32, i32)> {
    for (i, a) in input.iter().enumerate() {
        for b in input.iter().skip(i + 1) {
            if a + b == 2020 {
                return Some((*a, *b));
            }
        }
    }
    None
}

#[test]
fn example()  {
    let (a, b) = find_sum_2020(&input::EXAMPLE).unwrap();

    assert_eq!(a + b, 2020);
    assert_eq!(a * b, 514579);
}
