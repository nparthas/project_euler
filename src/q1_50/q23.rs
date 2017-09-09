pub fn q23() -> i64 {

    const LIMIT: i64 = 28123;
    let mut sum: i64 = 0;
    let mut abundant_vec: Vec<i64> = Vec::new();
    find_abundant_numbers(&mut abundant_vec, LIMIT);

    let mut abundant_sum_array: [bool; LIMIT as usize + 1] = [false; LIMIT as usize + 1];

    create_abundant_sum_array(&mut abundant_sum_array, &abundant_vec, LIMIT);

    for i in 1..LIMIT as usize {

        if !abundant_sum_array[i] {
            sum += i as i64;
        }
    }

    return sum;
}


fn find_abundant_numbers(vec: &mut Vec<i64>, limit: i64) {

    for i in 1..limit {
        if sum_proper_divisors(i) > i {
            vec.push(i);
        }
    }
}


fn sum_proper_divisors(num: i64) -> i64 {

    let mut i: i64 = 2;
    let mut upper: i64 = num;
    let mut sum: i64 = 1;

    while i < upper {
        if num % i == 0 {
            upper = num / i;
            sum += upper;
            if upper != i {
                sum += i;
            }
        }
        i += 1;
    }
    return sum;
}


fn create_abundant_sum_array(arr: &mut [bool], vec: &Vec<i64>, size: i64) {

    let mut i: usize = 0;
    let mut j: usize;

    while i < vec.len() as usize {
        j = i;
        while j < vec.len() as usize {

            if vec[i] + vec[j] <= size {
                arr[vec[i] as usize + vec[j] as usize] = true;
            } else {
                break;
            }

            j += 1;
        }
        i += 1;
    }

}
