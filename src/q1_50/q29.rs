use std::collections::HashMap;
extern crate num;
extern crate num_bigint;

use self::num_bigint::ToBigInt;
use std::ops::Mul;

pub fn q29() -> i64 {

    return compute_num_in_sequence(100, 100);
}

fn compute_num_in_sequence(a: i64, b: i64) -> i64 {

    let mut sequence = HashMap::new();

    for i in 2..a + 1 {
        for j in 2..b + 1 {
            let power = {
                let mut temp = i.to_bigint().unwrap();
                for _ in 1..j {
                    temp = temp.mul(i)
                }
                temp
            };
            sequence.insert(power, "");

        }
    }
    sequence.len() as i64
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_num_in_sequence_should_match_example() {
        println!("{}", compute_num_in_sequence(5, 5));
        assert_eq!(compute_num_in_sequence(5, 5), 15);
    }
}
