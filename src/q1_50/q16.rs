extern crate num;
extern crate num_bigint;

use self::num_bigint::{BigInt, ToBigInt};
use std::ops::Mul;


pub fn q16() -> i64 {

    let num_str: String = {
        let mut num: BigInt = 2.to_bigint().unwrap();
        for _ in 1..1000 {
            num = num.mul(2);
        }
        num.to_str_radix(10)
    };

    let mut num: BigInt = 2.to_bigint().unwrap();
    for _ in 1..1000 {
        num = num.mul(2);
    }

    let mut dig_sum: i64 = 0;

    for c in num_str.chars() {
        dig_sum += c.to_digit(10).unwrap() as i64;
    }

    return dig_sum;
}
