extern crate num;
extern crate num_bigint;

use self::num_bigint::{BigInt, ToBigInt};
use std::ops::Div;

pub fn q25() -> i64 {

    let mut f1: BigInt = 1.to_bigint().unwrap();
    let mut f2: BigInt = 1.to_bigint().unwrap();

    let mut limit: BigInt = 10.to_bigint().unwrap();

    for _ in 1..999 {
        limit = limit * 10;
    }

    let mut temp: BigInt;
    let mut count: i64 = 1;

    while f1.clone().div(limit.clone()) == 0.to_bigint().unwrap() {

        temp = f1;
        f1 = f2.clone();
        f2 = f2 + temp;

        count += 1;
    }


    return count;
}
