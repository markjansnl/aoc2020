use aoc18::input;

fn evaluate(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .fold(vec![(vec![0u64], b'+')], |mut stack, byte| {
                    match byte {
                        b' ' => {}
                        b'+' | b'*' => {
                            stack.last_mut().unwrap().1 = byte;
                        }
                        b'(' => {
                            stack.push((vec![0u64], b'+'));
                        }
                        b')' => {
                            let (factors2, _) = stack.pop().unwrap();
                            let (factors1, operator) = stack.last_mut().unwrap();
                            let value1 = factors1.last().unwrap();
                            let value2 = factors2.iter().product();
                            if *operator == b'+' {
                                *factors1.last_mut().unwrap() = value1 + value2;
                            } else {
                                factors1.push(value2);
                            };
                        }
                        _ => {
                            let (factors1, operator) = stack.last_mut().unwrap();
                            let value1 = factors1.last().unwrap();
                            let value2 = byte as u64 - 48;
                            if *operator == b'+' {
                                *factors1.last_mut().unwrap() = value1 + value2;
                            } else {
                                factors1.push(value2);
                            };
                        }
                    }
                    stack
                })[0]
                .0
                .iter()
                .product::<u64>()
        })
        .sum()
}

fn main() {
    println!("{}", evaluate(input::USER));
}

#[test]
fn test_part2_example() {
    assert_eq!(evaluate("1 + 2 * 3 + 4 * 5 + 6"), 231);
    assert_eq!(evaluate("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    assert_eq!(evaluate("2 * 3 + (4 * 5)"), 46);
    assert_eq!(evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
    assert_eq!(
        evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
        669060
    );
    assert_eq!(
        evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        23340
    );
}
