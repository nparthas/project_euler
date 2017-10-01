pub fn q27() -> i64 {

    let mut max_count = -1;
    let mut prod: i64 = -1;
    let prime_vec: Vec<i64> = generate_prime_list();

    for a in -999..1000 {
        for b in -999..1000 {

            let count = count_consecutive(a, b);
            if count > max_count {
                max_count = count;
                prod = a * b;
            }
        }
    }
    prod
}

fn generate_prime_list() -> Vec<i64> {
    let mut vec: Vec<i64> = Vec::new();

    for i in 2..1000 {
        if is_prime(i) {
            vec.push(i);
            vec.push(-i);
        }
    }
    vec
}

fn count_consecutive(a: i64, b: i64) -> i64 {

    if !is_prime(b) {
        return 0;
    }

    if (1 + a + b) % 2 != 1 {
        return 1;
    }

    let mut n: i64 = 0;

    while is_prime(n.pow(2) + a * n + b) {
        n += 1;
    }

    n
}


fn is_prime(n: i64) -> bool {

    let num = n.abs();
    let mut flag: bool = true;
    for i in 2..((num as f64).sqrt() as i64 + 1) {
        if num % i == 0 {
            flag = false;
        }
    }
    flag
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn count_consecutive_should_match_examples() {

        assert_eq!(count_consecutive(1, 41), 40);
        assert_eq!(count_consecutive(-79, 1601), 80);
    }


    #[test]
    fn prime_tests() {
        // should be prime
        assert!(is_prime(2));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(is_prime(11));
        assert!(is_prime(13));
        assert!(is_prime(97));
        assert!(is_prime(-3));
        assert!(is_prime(-7));
    }

    #[test]
    fn shoud_not_be_prime() {

        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(15), false);
        assert_eq!(is_prime(18), false);
        assert_eq!(is_prime(27), false);
        assert_eq!(is_prime(1681), false);
        assert_eq!(is_prime(-15), false);
        assert_eq!(is_prime(-50), false);
    }
}
