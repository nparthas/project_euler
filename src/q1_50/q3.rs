// use std::ops::Rem;

pub fn q2() -> i64 {

    let num = 600851475143_i64;
    let mut largest_num = 0_i64;

    for i in 1..(num as f64).sqrt() as i64 {
        if num % i == 0 {
            let mut flag = true;
            for j in 2..i {
                if i % j == 0 {
                    flag = false;
                }
            }
            if flag {
                largest_num = i;
            }
        }
    }
    largest_num
}
