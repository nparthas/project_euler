pub fn q3()-> i64 {
	
	let mut max:i64 = 100001;

	for i in (111..999).rev() {

		if max >= i*999 {
			break;
		}
		for j in (111..999).rev() {
			let num = i * j;
			if num > max && is_palendrome(num) {
				max = num;
			} 
		}

	}
	return max;
}

fn is_palendrome(num:i64)-> bool {

	let s: String = num.to_string();
	let half = s.len()/2;
	s.bytes().take(half).eq(s.bytes().rev().take(half))
}