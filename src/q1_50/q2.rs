pub fn q2() -> i64 {

    let mut f1: i64 = 1;
    let mut f2: i64 = 2;
    let mut total: i64 = 0;

    while f1 < 4000000 {

        if f2 % 2 == 0 {
            total += f2;
        }

        f2 ^= f1;
        f1 ^= f2;
        f2 ^= f1;
        f2 += f1;

    }

    return total;
}
