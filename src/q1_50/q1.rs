pub fn q1() -> i64 {

    sum_all_n(3) + sum_all_n(5) - sum_all_n(15)

}


fn sum_all_n(n: i64) -> i64 {

    let p = 999 / n;
    return n * p * (p + 1) / 2;
}
