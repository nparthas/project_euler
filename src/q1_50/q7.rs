pub fn q7() -> i64 {

    let mut count = 0;
    let mut num: i64 = 1;

    while count < 10001 {

        num += 1;

        if is_prime(num) {
            count += 1;
        }
    }
    return num;
}


fn is_prime(num: i64) -> bool {

    for i in 2..(num as f64).sqrt() as i64 + 1 {

        if num % i == 0 {
            return false;
        }

    }
    return true;
}
