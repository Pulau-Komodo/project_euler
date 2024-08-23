fn main() {
	let mut largest_palindrome = 0;
	for a in (100..=999).rev() {
		for b in (a..=999).rev() {
			let product = a * b;
			println!("{a} * {b} = {product}");
			if product == 872496 {
				return;
			}
			if is_palindrome(product) && product > largest_palindrome {
				largest_palindrome = product;
			}
		}
	}
	println!("{largest_palindrome}")
}

fn is_palindrome(mut num: u32) -> bool {
	let mut digits = [0; 6];
	for digit in &mut digits {
		*digit = num % 10;
		num /= 10;
	}
	let zeroes = digits
		.into_iter()
		.rev()
		.position(|digit| digit > 0)
		.unwrap();
	let digits = &digits[0..digits.len() - zeroes];
	for i in 0..digits.len() / 2 {
		if digits[i] != digits[digits.len() - 1 - i] {
			return false;
		}
	}
	true
}
