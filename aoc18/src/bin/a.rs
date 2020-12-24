use aoc18::input;

fn evaluate(input: &str) -> u64 {
    println!();
    input.lines().map(|line| line.bytes().fold(vec![(0u64, b'+')], |mut stack, byte| {
        let (value1, operator) = stack.pop().unwrap();
        match byte {
            b' ' => {
                stack.push((value1, operator));
            },
            b'(' => {
                stack.push((value1, operator));
                stack.push((0, b'+'));
            },
            b')' => {
                let (value2, operator2) = stack.pop().unwrap();
                stack.push((match operator2 {
                    b'+' => value1 + value2,
                    b'*' => value1 * value2,
                    _    => panic!("unknown operator"),
                }, 0));
            },
            b'+' | b'*' => {
                stack.push((value1, byte));
            },
            b'\n' => {
                stack.push((value1, b'+'));
            },
            _ => {
                let value2 = byte as u64 - 48;
                stack.push((match operator {
                    b'+' => value1 + value2,
                    b'*' => value1 * value2,
                    _    => panic!("unknown operator"),
                }, 0));
            }
        }
        stack
    })[0].0).sum()
}

fn main() {
    println!("{}", evaluate(input::USER));
}

#[test]
fn test_example() {
    // assert_eq!(evaluate("1 + 2 * 3 + 4 * 5 + 6"), 71);
    assert_eq!(evaluate("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    // assert_eq!(evaluate("2 * 3 + (4 * 5)"), 26);
    // assert_eq!(evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
    // assert_eq!(evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
    // assert_eq!(evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);    
}
