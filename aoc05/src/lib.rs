pub mod input;

pub fn seat_id(boarding_pass: &str) -> u16 {
    boarding_pass.bytes().fold(0, |mut acc, byte| {
        acc = acc << 1;
        if byte == b'B' || byte == b'R' {
            acc += 1;
        }
        acc
    })
}

#[test]
fn test_seat_id() {
    assert_eq!(seat_id("BFFFBBFRRR"), 567);
    assert_eq!(seat_id("FFFBBBFRRR"), 119);
    assert_eq!(seat_id("BBFFBBFRLL"), 820);
}
