extern crate num;
extern crate num_bigint;

use self::num_bigint::{BigInt, ToBigInt};
use std::ops::Mul;

pub fn q20() -> i64 {

    let num_str: String = {
        let mut num: BigInt = 1.to_bigint().unwrap();
        for i in 2..101 {
            num = num.mul(i);
        }
        num.to_str_radix(10)
    };

    let mut dig_sum: i64 = 0;

    for c in num_str.chars() {
        dig_sum += c.to_digit(10).unwrap() as i64;
    }

    return dig_sum;
}
