use aoc08::{input, parse_input, GameConsole, RunResult};

fn main() {
    println!("{}", run_game_console(input::USER));
}

fn run_game_console(input: &str) -> isize {
    let instructions_base = parse_input(input);
    for index in 0..instructions_base.len() {
        let mut instructions = instructions_base.clone();
        if !instructions[index].replace_jmp_nop() {
            continue;
        }
        let mut game_console = GameConsole::new(&instructions);
        if let RunResult::EndOfInstructions = game_console.run() {
            return game_console.acc();
        }
    }
    0
}

#[test]
fn test_example() {
    assert_eq!(run_game_console(input::EXAMPLE), 8);
}
