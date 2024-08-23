fn main() {
	let mut sum = 0;
	const LIMIT: u32 = 4_000_000;
	let mut a = 1;
	let mut b = 1;
	loop {
		let c = a + b;
		a = b;
		b = c;
		if b > LIMIT {
			return println!("{sum}");
		}
		if b % 2 == 0 {
			sum += b;
		}
	}
}
