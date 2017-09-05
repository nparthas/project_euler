pub fn q21() -> i64 {

    let mut sum: u32 = 0;

    for i in 2..10000 {
        let num: u32 = find_wouldbe_pair(i);

        if num > i && find_wouldbe_pair(num) == i {
            sum += i;
            sum += num;
        }
    }
    return sum as i64;
}


fn find_wouldbe_pair(num: u32) -> u32 {

    let mut sum: u32 = 1;

    for i in 2..(num as f64).sqrt() as u32 {
        if num % i == 0 {
            sum += i + num / i;
        }
    }


    return sum;
}
