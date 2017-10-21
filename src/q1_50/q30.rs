pub fn q30() -> i64 {

    let mut sum_of_matches: i64 = 0;

    for i in 2..354294 {
        let mut sum_powers = 0;
        let mut num = i;

        while num > 0 {
            sum_powers += (num as i64 % 10).pow(5);
            num /= 10;
        }

        if sum_powers == i {
            sum_of_matches += i;
        }
    }
    sum_of_matches
}
