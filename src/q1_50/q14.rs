pub fn q14() -> i64 {

    let mut longest_count: usize = 0;
    let mut longest_staring: usize = 1;
    let mut i: usize = 1;

    while i < 1000000 {
        let mut j = i;
        let mut count: usize = 0;
        loop {
            if j == 1 {
                break;
            }

            j = match j % 2 {
                0 => j / 2,
                _ => 3 * j + 1,
            };
            count += 1;
        }

        if count > longest_count {
            longest_count = count;
            longest_staring = i;
        }
        i += 2;
    }
    return longest_staring as i64;
}
