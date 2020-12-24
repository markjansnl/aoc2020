use aoc18::input;

fn evaluate(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            line.bytes().fold(vec![(0u64, b'+')], |mut stack, byte| {
                match byte {
                    b' ' => {}
                    b'(' => {
                        stack.push((0, b'+'));
                    }
                    b')' => {
                        let (value1, _) = stack.pop().unwrap();
                        let (value2, operator2) = stack.last().unwrap();
                        stack.last_mut().unwrap().0 = if *operator2 == b'+' { value1 + value2 } else { value1 * value2 };
                    }
                    b'+' | b'*' => {
                        stack.last_mut().unwrap().1 = byte;
                    }
                    _ => {
                        let (value1, operator) = stack.last().unwrap();
                        let value2 = byte as u64 - 48;
                        stack.last_mut().unwrap().0 = if *operator == b'+' { value1 + value2 } else { value1 * value2 };
                    }
                }
                stack
            })[0]
                .0
        })
        .sum()
}

fn main() {
    println!("{}", evaluate(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(evaluate("1 + 2 * 3 + 4 * 5 + 6"), 71);
    assert_eq!(evaluate("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    assert_eq!(evaluate("2 * 3 + (4 * 5)"), 26);
    assert_eq!(evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
    assert_eq!(evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
    assert_eq!(
        evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        13632
    );
}
