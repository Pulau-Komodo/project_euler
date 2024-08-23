use std::{array, fs};

fn main() {
	let file = fs::read_to_string("./src/bin/54.txt").unwrap();
	let count = file
		.lines()
		.map(|line| {
			let mut iterator = line.split(' ').map(str::as_bytes);
			(
				Hand::from_iterator(&mut iterator),
				Hand::from_iterator(&mut iterator),
			)
		})
		//.skip(2)
		//.take(1)
		.filter(|(a, b)| a > b)
		.count();
	println!("{count}");
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
	cards: [Card; 5],
}

impl Hand {
	fn from_iterator<'a>(iterator: &mut impl Iterator<Item = &'a [u8]>) -> Self {
		let mut cards = array::from_fn(|_| {
			let slice = iterator.next().unwrap();
			Card::from_bytes(slice[1], slice[0])
		});
		let cards_copy = cards.clone();
		cards.sort_unstable_by_key(|card| {
			(
				cards_copy
					.iter()
					.filter(|other_card| other_card.value == card.value)
					.count(),
				card.value,
			)
		});
		cards.reverse();
		Self { cards }
	}
	fn evaluate(&self) -> (Rank, &[Card; 5]) {
		use Rank::*;

		let all_same_suit = self
			.cards
			.iter()
			.skip(1)
			.all(|card| card.suit == self.cards[0].suit);
	
		let highest_value = self.cards.iter().map(|card| card.value).max().unwrap();
		let all_consecutive = highest_value > 5
			&& (highest_value - 4..highest_value)
				.all(|value| self.cards.iter().any(|card| card.value == value));
		
		if all_same_suit && all_consecutive {
			return (StraightFlush, &self.cards);
		}
		let same_value_count = self.cards[1..]
			.iter()
			.take_while(|card| card.value == self.cards[0].value)
			.count() + 1;
		if same_value_count == 4 {
			return (FourOfAKind, &self.cards);
		}
		if same_value_count == 3 {
			let others_paired = self.cards[3].value == self.cards[4].value;
			if others_paired {
				return (FullHouse, &self.cards);
			}
		}
		let same_suit = self.cards[1..]
			.iter()
			.all(|card| card.suit == self.cards[0].suit);
		if same_suit {
			return (Flush, &self.cards);
		}
		if all_consecutive {
			return (Straight, &self.cards);
		}
		if same_value_count == 3 {
			return (ThreeOfAKind, &self.cards);
		}
		if same_value_count == 2 {
			let another_pair = self.cards[2].value == self.cards[3].value;
			if another_pair {
				return (TwoPairs, &self.cards);
			}
			return (Pair, &self.cards);
		}

		(Nothing, &self.cards)
	}
}

impl PartialOrd for Hand {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Hand {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		//dbg!(dbg!(self.evaluate()).cmp(dbg!(&other.evaluate())))
		self.evaluate().cmp(&other.evaluate())
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
	Nothing,
	Pair,
	TwoPairs,
	ThreeOfAKind,
	Straight,
	Flush,
	FullHouse,
	FourOfAKind,
	StraightFlush,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {
	value: u8,
	suit: u8,
}

impl Card {
	fn from_bytes(suit: u8, value: u8) -> Self {
		let value = match value {
			b'T' => 10,
			b'J' => 11,
			b'Q' => 12,
			b'K' => 13,
			b'A' => 14,
			num => num - b'0',
		};
		Self { suit, value }
	}
}

impl PartialOrd for Card {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Card {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.value.cmp(&other.value)
	}
}
