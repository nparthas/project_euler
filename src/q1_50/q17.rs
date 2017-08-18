pub fn q17() -> i64 {
    //three hundred and forty-two -> 23
    // count 1-9 11-19 20,30,40 100 and
    let mut count: i64 = 0;

    count += count_1_9();
    count += count_11_19();
    count += count_mul_10s();
    count += count_100s();
    count += count_100_prefix();
    count += count_ands();

    count += 3 + 8;

    return count;
}


fn count_1_9() -> i64 {

    let dig_str: &str = "onetwothreefourfivesixseveneightnine";
    let str_len: usize = dig_str.len();
    (str_len * 9 * 10) as i64
}


fn count_11_19() -> i64 {

    let teen_str: &str = "teneleventwelvethirteenfourteenfifteen\
    sixteenseventeeneighteennineteen";
    let str_len: usize = teen_str.len();
    (str_len * 10) as i64
}


fn count_mul_10s() -> i64 {

    let tens_str: &str = "twentythirtyfortyfiftysixtyseventyeightyninety";
    let str_len: usize = tens_str.len();
    (str_len * 10 * 10) as i64
}


fn count_100s() -> i64 {

    let sum: u16 = 7;
    (sum * 100 * 9) as i64
}


fn count_100_prefix() -> i64 {

    let prefix_str = "onetwothreefourfivesixseveneightnine";
    let str_len: usize = prefix_str.len();
    (str_len * 100) as i64
}


fn count_ands() -> i64 {

    let sum: u16 = 3;
    (sum * 9 * 99) as i64

}
