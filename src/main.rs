#![allow(dead_code)]
use std::time::Instant;

mod q1_50;
mod q51_100;

fn run_and_print(function: &Fn() -> i64) -> () {
    let now = Instant::now();

    let result: i64 = function();

    println!(
        "In {0} seconds: {1}",
        now.elapsed().as_secs() as f64 + now.elapsed().subsec_nanos() as f64 * 1e-9,
        result
    );
}

fn main() {

    run_and_print(&q1_50::q24::q24);


}
