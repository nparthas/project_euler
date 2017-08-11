pub fn q10() -> i64 {

    const LIMIT: usize = 2000000;
    const ARRAY_BOUND: usize = (LIMIT - 1) / 2;
    let mut prime_array: [bool; ARRAY_BOUND] = [false; ARRAY_BOUND];

    let x_limit: i64 = ((LIMIT as f64).sqrt() as i64 - 1) / 2;

    for i in 1..x_limit {

        if !prime_array[i as usize] {

            let mut j = 2 * i * (i + 1);
            while j < ARRAY_BOUND as i64 {
                prime_array[j as usize] = true;
                j += 2 * i + 1;
            }
        }
    }

    let mut sum: i64 = 2;

    for i in 1..ARRAY_BOUND as i64 {
        if !prime_array[i as usize] {
            sum += 2 * i + 1;
        }
    }
    return sum;
}


pub fn q10_trivial() -> i64 {

    fn is_prime(num: i64) -> bool {

        for i in 2..(num as f64).sqrt() as i64 + 1 {
            if num % i == 0 {
                return false;
            }

        }
        return true;
    }

    let mut sum: i64 = 0;
    for i in 2..2000000 {
        if is_prime(i) {
            println!("{}", i);
            sum += i;
        }

    }

    return sum;
}
