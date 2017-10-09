pub fn q26() -> i64 {

    let mut length: i64 = -1;

    for i in (1..1001).rev() {

        if length >= i {
            break;
        }

        let mut remainders: Vec<i64> = vec![0; i as usize];
        let mut value: usize = 1;
        let mut position: i64 = 0;

        while remainders[value] == 0 && value != 0 {
            remainders[value] = position;
            value *= 10;
            value %= i as usize;
            position += 1;
        }

        if position - remainders[value] > length {
            length = position - remainders[value]
        }

    }
    length + 1 // the number with the longest cycle is one bigger than the length
}
