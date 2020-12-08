use aoc08::{input, parse_input, GameConsole};

fn main() {
    println!("{}", run_game_console(input::USER));
}

fn run_game_console(input: &str) -> isize {
    let instructions = parse_input(input);
    let mut game_console = GameConsole::new(&instructions);
    game_console.run();
    game_console.acc()
}

#[test]
fn test_example() {
    assert_eq!(run_game_console(input::EXAMPLE), 5);
}
