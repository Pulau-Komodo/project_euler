fn main() {
	for n in (1_u32..).filter(|n| is_prime(*n)) {
		let length = n.ilog10() + 1;
		let digits = split_into_digits(n);
		for mask in 1..2_u32.pow(length) - 2 {
			let mut prime_count = 0;
			for new_digit in 0..=9 {
				let mut digits = digits.clone();
				for digit in mask_iterator(mask, length)
					.zip(&mut digits)
					.filter_map(|(condition, digit)| condition.then_some(digit))
				{
					*digit = new_digit;
				}
				let new_number = merge_from_digits(&digits);
				if new_number.checked_ilog10().unwrap_or(0) + 1 == length && is_prime(new_number) {
					prime_count += 1;
				}
			}
			if prime_count >= 8 {
				return println!("{n}");
			}
		}
	}
}

fn is_prime(num: u32) -> bool {
	for divisor in 2..(num as f32).sqrt() as u32 {
		if num % divisor == 0 {
			return false;
		}
	}
	true
}

fn mask_iterator(mut mask: u32, length: u32) -> impl Iterator<Item = bool> {
	(0..length).map(move |_| {
		let result = mask % 2 != 0;
		mask /= 2;
		result
	})
}

fn split_into_digits(mut num: u32) -> Vec<u8> {
	let mut digits = Vec::with_capacity(num.ilog10() as usize);
	while num > 0 {
		digits.push((num % 10) as u8);
		num /= 10;
	}
	digits
}

fn merge_from_digits(digits: &[u8]) -> u32 {
	let mut num = 0;
	for &digit in digits.iter().rev() {
		num *= 10;
		num += digit as u32;
	}
	num
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_name() {
		assert_eq!(523, merge_from_digits(&split_into_digits(523)))
	}
}
