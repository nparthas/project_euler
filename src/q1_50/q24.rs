pub fn q24() -> i64 {

    let mut arr: [i64; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for _ in 1..1000000 {
        next_permutation(&mut arr);
    }

    let mut sum: i64 = 0;
    let mut index: i32 = 9;

    for i in arr.iter() {

        sum += i * 10_i64.pow(index as u32);
        index -= 1;

    }
    return sum;
}


fn next_permutation(array: &mut [i64]) -> bool {

    if array.len() == 0 {
        return false;
    }
    let mut i: usize = array.len() - 1;
    while i > 0 && array[i - 1] >= array[i] {
        i -= 1;
    }
    if i == 0 {
        return false;
    }

    let mut j: usize = array.len() - 1;
    while array[j] <= array[i - 1] {
        j -= 1;
    }
    array.swap(i - 1, j);

    array[i..].reverse();
    true
}
