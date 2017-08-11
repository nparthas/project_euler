pub fn q4() -> i64 {

    let mut primes = Vec::new();

    for i in 2..21 {
        let mut flag = true;
        for j in 2..(i as f64).sqrt() as i64 + 1 {
            if i % j == 0 {
                flag = false;
            }
        }
        if flag {
            primes.push(i);
        }
    }

    let mut num: i64 = 1;
    for i in 2..21 {
        num *= i;
    }

    for prime in primes {

        let mut flag = true;
        while flag {

            if is_divisible1_20(num / prime) {
                num /= prime;
            } else {
                flag = false;
            }

        }
    }

    return num;
}

fn is_divisible1_20(num: i64) -> bool {

    let mut flag = true;
    for i in 2..21 {
        if num % i != 0 {
            flag = false;
        }
    }
    return flag;

}
