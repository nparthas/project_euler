pub fn q12() -> i64 {

    fn count_divisors(num: i64) -> i64 {

        let mut count: i64 = 0;
        for i in 1..(num as f64).sqrt() as i64 {
            if num % i == 0 {
                count += 2;
            }
        }

        if ((num as f64).sqrt() as i64).pow(2) == num {
            count -= 1;
        }
        return count;
    }

    let mut triangle_num: i64 = 1;

    for i in 2..<i64>::max_value() {

        triangle_num += i;
        if triangle_num % 2 == 0 && count_divisors(triangle_num) > 500 {
            return triangle_num;
        }
    }
    return -1;
}
