pub fn q6()-> i64 {

	let num = 100;
	let mut sum_of_sq:i64 = 0;
	let mut sum_squared:i64 = 0;

	for i in 1..num+1 {
		sum_of_sq += (i as i64).pow(2);
		sum_squared += i;
	}
	sum_squared = sum_squared.pow(2);

	return sum_squared - sum_of_sq;
}