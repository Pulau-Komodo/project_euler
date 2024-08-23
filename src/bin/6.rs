fn main() {
	let sum_of_squares: u32 = (1_u32..=100).map(|n| n.pow(2)).sum();
	let square_of_sums = (100_u32 * 101 / 2).pow(2);
	println!("{}", square_of_sums - sum_of_squares);
}
