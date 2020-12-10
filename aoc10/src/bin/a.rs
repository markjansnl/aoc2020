use aoc10::input;

fn main() {
    println!("{}", jolt_differences(input::USER));
}

fn jolt_differences(input: &str) -> usize {
    let mut adapters: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    adapters.push(0);
    adapters.sort();

    let jolt_differences =
        adapters[1..]
            .iter()
            .enumerate()
            .fold([0; 3], |mut acc, (prev, jolt)| {
                acc[*jolt - adapters[prev] - 1] += 1;
                acc
            });

    jolt_differences[0] * (jolt_differences[2] + 1)
}

#[test]
fn test_example1() {
    assert_eq!(jolt_differences(input::EXAMPLE1), 7 * 5);
}

#[test]
fn test_example2() {
    assert_eq!(jolt_differences(input::EXAMPLE2), 22 * 10);
}
