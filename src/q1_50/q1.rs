pub fn q1()-> i64 {

	sum_all_n(3) + sum_all_n(5) - sum_all_n(15)

}


fn sum_all_n(n:i64)-> i64 {

	let limit = 999/n;
	let mut total:i64 = 0;
	for i in 1..limit + 1 {
		total += n * i;
	}

	return total;
}