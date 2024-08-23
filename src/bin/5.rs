fn main() {
	for n in 1.. {
		if (1..=20).all(|divisor| n % divisor == 0) {
			return println!("{n}");
		}
	}
}
