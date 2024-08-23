fn main() {
	const NUM: u64 = 600_851_475_143;
	for divisor in 2.. {
		if NUM % divisor == 0 && is_prime(NUM / divisor) {
			return println!("{}", NUM / divisor);
		}
	}
}

fn is_prime(num: u64) -> bool {
	for divisor in 2..(num as f32).sqrt() as u64 {
		if num % divisor == 0 {
			return false;
		}
	}
	true
}
