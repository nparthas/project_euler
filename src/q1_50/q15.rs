pub fn q15() -> i64 {

    let mut num: f64 = 1_f64;
    for i in 1..21 {
        let j: f64 = i as i32 as f64;
        num *= (20.0 + j) / j;
    }
    let result: i64 = num.round() as i64;

    return result;
}



pub fn q15_only_ints() -> i64 {

    let mut num: i64 = 1;
    let mut divisor: i64 = factorial(20);
    let mut prime_array: [i64; 19] = [1; 19];

    {
        prime_array[0] = 2;
        let mut curr_prime: i64 = 3;
        for i in 1..18 {
            prime_array[i] = curr_prime;
            curr_prime = next_prime(curr_prime);
        }
    }

    for i in 21..41 {
        num *= i;

        if i % 10 == 0 {
            simplify(&mut num, &mut divisor, &prime_array);
        }
    }


    simplify(&mut num, &mut divisor, &prime_array);

    return num; // 40! / 20! 20 ! -> 21..40 / 20!
}

fn factorial(num: usize) -> i64 {
    return {
        let mut temp: i64 = num as i64;
        for i in 2..num {
            temp *= i as i64;
        }
        temp
    };
}

fn is_prime(num: i64) -> bool {
    for i in 2..(num as f64).sqrt() as i64 + 1 {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}

fn next_prime(num: i64) -> i64 {
    let mut temp: i64 = num + 2;
    while !is_prime(temp) {
        temp += 2;
    }
    temp
}


fn simplify(numer: &mut i64, denom: &mut i64, prime_array: &[i64; 19]) {
    for i in 0..18 {
        while *numer % prime_array[i] == 0 && *denom % prime_array[i] == 0 {
            *numer /= prime_array[i];
            *denom /= prime_array[i];
        }
    }
}
