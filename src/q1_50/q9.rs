pub fn q9() -> i64 {

    let sum = 1000;
    for a in 3..(sum - 3) / 3 {
        for b in a + 1..(sum - 1 - a) / 2 {
            let c = sum - a - b;
            if (a as i32).pow(2) + (b as i32).pow(2) == (c as i32).pow(2) {
                println!("{0}, {1}, {2}", a, b, c);
                return a * b * c;
            }

        }
    }
    return -1;
}

pub fn q9_simple() -> i64 {

    for c in 1..1000 {
        for b in 1..1000 {
            for a in 1..1000 {
                if a + b + c == 1000 {
                    if (a as i32).pow(2) + (b as i32).pow(2) == (c as i32).pow(2) {
                        println!("{0}, {1}, {2}", a, b, c,);
                        return a * b * c;
                    }


                }
            }

        }

    }

    return -1;

}
