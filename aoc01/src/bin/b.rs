// use rayon::prelude::*;
use aoc01::input;

fn main() {
    if let Some((a, b, c)) = find_sum_2020(&input::USER) {
        println!("{}", a * b * c);
    }
}

fn find_sum_2020(input: &[i32]) -> Option<(i32, i32, i32)> {
    for (i, a) in input.iter().enumerate() {
        for (j, b) in input[i + 1..].iter().enumerate() {
            for c in input[j + 1..].iter() {
                if a + b + c == 2020 {
                    return Some((*a, *b, *c));
                }
            }
        }
    }
    None
}

// Commented out, because parallel is not faster for such a small computation
//
// fn _find_sum_2020_par(input: &[i32]) -> Option<(i32, i32, i32)> {
//     input.par_iter().enumerate().find_map_any(|(i, a)| {
//         input.par_iter().enumerate().skip(i + 1).find_map_any(|(j, b)| {
//             input.par_iter().skip(j + 1).find_map_any(|c| {
//                 if a + b + c == 2020 {
//                     Some((*a, *b, *c))
//                 } else {
//                     None
//                 }
//             })
//         })
//     })
// }

#[test]
fn example() {
    let (a, b, c) = find_sum_2020(&input::EXAMPLE).unwrap();

    assert_eq!(a + b + c, 2020);
    assert_eq!(a * b * c, 241861950);
}
