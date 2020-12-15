pub mod input;

#[derive(Clone, Copy)]
pub enum Schedule {
    Bus(usize),
    NoRestrictions,
}

pub fn parse_input(input: &str) -> (usize, Vec<Schedule>) {
    let mut lines = input.lines();
    let timestamp = lines.next().unwrap().parse().unwrap();
    let schedules = lines
        .next()
        .unwrap()
        .split(",")
        .map(|schedule| match schedule {
            "x" => Schedule::NoRestrictions,
            _ => Schedule::Bus(schedule.parse().unwrap()),
        })
        .collect();

    (timestamp, schedules)
}
