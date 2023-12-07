use rustvent2023::get_input;
use std::cmp::{Ord, Ordering};
use std::collections::BTreeMap;
use std::{time, usize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(c: char) -> Option<Card> {
        use Card::*;
        match c {
            '2' => Some(N2),
            '3' => Some(N3),
            '4' => Some(N4),
            '5' => Some(N5),
            '6' => Some(N6),
            '7' => Some(N7),
            '8' => Some(N8),
            '9' => Some(N9),
            'T' => Some(T),
            'J' => Some(J),
            'Q' => Some(Q),
            'K' => Some(K),
            'A' => Some(A),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card2 {
    J,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    Q,
    K,
    A,
}

impl Card2 {
    fn from_char(c: char) -> Option<Card2> {
        use Card2::*;
        match c {
            '2' => Some(N2),
            '3' => Some(N3),
            '4' => Some(N4),
            '5' => Some(N5),
            '6' => Some(N6),
            '7' => Some(N7),
            '8' => Some(N8),
            '9' => Some(N9),
            'T' => Some(T),
            'J' => Some(J),
            'Q' => Some(Q),
            'K' => Some(K),
            'A' => Some(A),
            _ => None,
        }
    }
}

// See this discussion: https://users.rust-lang.org/t/how-to-sort-enum-variants/52291/7
#[derive(Debug, Copy, Clone)]
struct AllwaysEqual<T>(T);

impl<T> From<T> for AllwaysEqual<T> {
    // Required method
    fn from(value: T) -> Self {
        AllwaysEqual(value)
    }
}

impl<T> PartialEq for AllwaysEqual<T> {
    // Required method
    fn eq(&self, other: &AllwaysEqual<T>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<T> Eq for AllwaysEqual<T> {}

impl<T> PartialOrd for AllwaysEqual<T> {
    // Required method
    fn partial_cmp(&self, other: &AllwaysEqual<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for AllwaysEqual<T> {
    // Panics for two equal hands (yes, this is not clean)
    fn cmp(&self, other: &AllwaysEqual<T>) -> Ordering {
        Ordering::Equal
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Set<T> {
    High(AllwaysEqual<T>),
    OnePair(AllwaysEqual<T>),
    TwoPair(AllwaysEqual<(T, T)>),
    ThreeOAK(AllwaysEqual<T>),
    FullHouse(AllwaysEqual<(T, T)>),
    FourOAK(AllwaysEqual<T>),
    FiveOAK(AllwaysEqual<T>),
}

impl<T: Ord + Copy> Set<T> {
    fn from_cards(cards: &Vec<T>) -> Set<T> {
        use Set::*;
        let mut counts = BTreeMap::new();
        for card in cards {
            *(counts.entry(card).or_insert(0)) += 1;
        }

        let find_count = |count| counts.iter().find(|(_, _c)| **_c == count);
        let find_pairs = || -> Option<_> {
            let pairs: Vec<T> = counts
                .iter()
                .filter_map(|(card, count)| match count {
                    2 => Some(**card),
                    _ => None,
                })
                .collect();
            if pairs.len() > 0 {
                Some(pairs)
            } else {
                None
            }
        };

        if let Some((card, _)) = find_count(5) {
            FiveOAK((**card).into())
        } else if let Some((card, _)) = find_count(4) {
            FourOAK((**card).into())
        // chained if let are not yet supported (see issue 53667 or eRFC 2497)
        // }else if let Some((card1, _)) = find_count(3) && let Some((card2, _)) = find_count(2) {
        } else if let Some((card1, _)) = find_count(3) {
            if let Some((card2, _)) = find_count(2) {
                FullHouse((**card1, **card2).into())
            } else {
                ThreeOAK((**card1).into())
            }
        } else if let Some(cards) = find_pairs() {
            if cards.len() == 2 {
                TwoPair((cards[0], cards[1]).into())
            } else {
                OnePair((cards[0]).into())
            }
        } else {
            High((*cards.iter().max().unwrap()).into())
        }
    }
}

#[derive(Debug, Clone, Eq)]
struct Hand<T: Ord> {
    cards: Vec<T>,
    bid: usize,
    set: Set<T>,
}

impl<T: Eq + Ord> PartialEq for Hand<T> {
    // Required method
    fn eq(&self, other: &Hand<T>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<T: Eq + Ord> PartialOrd for Hand<T> {
    // Required method
    fn partial_cmp(&self, other: &Hand<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq + Ord> Ord for Hand<T> {
    // Panics for two equal hands (yes, this is not clean)
    fn cmp(&self, other: &Hand<T>) -> Ordering {
        use Ordering::*;
        match self.set.cmp(&other.set) {
            Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .map(|(h1, h2)| h1.cmp(h2))
                .filter(|o| *o != Equal)
                .next()
                .unwrap(),
            other_ord => other_ord,
        }
    }
}

fn parse(input: &str) -> Vec<Hand<Card>> {
    input
        .lines()
        .map(|l| {
            let mut iter = l.split_whitespace();
            let cards = iter
                .next()
                .unwrap()
                .chars()
                .filter_map(Card::from_char)
                .collect();
            let bid = iter.next().unwrap().parse().unwrap();

            let set = Set::from_cards(&cards);
            Hand { cards, bid, set }
        })
        .collect()
}

const VALID_CARD2: [Card2; 12] = [
    Card2::N2,
    Card2::N3,
    Card2::N4,
    Card2::N5,
    Card2::N6,
    Card2::N7,
    Card2::N8,
    Card2::N9,
    Card2::T,
    Card2::Q,
    Card2::K,
    Card2::A,
];

fn parse2(input: &str) -> Vec<Hand<Card2>> {
    use Card2::*;
    input
        .lines()
        .map(|l| {
            let mut iter = l.split_whitespace();
            let cards: Vec<_> = iter
                .next()
                .unwrap()
                .chars()
                .filter_map(Card2::from_char)
                .collect();
            let bid = iter.next().unwrap().parse().unwrap();

            let ji: Vec<_> = cards
                .iter()
                .enumerate()
                .filter_map(|(i, c)| if *c == Card2::J { Some(i) } else { None })
                .collect();
            let mut all_combinations: Vec<Vec<_>> =
                Vec::with_capacity((12 as usize).pow(ji.len() as u32));
            all_combinations.push(cards.clone());

            for i in ji {
                let started: Vec<_> = all_combinations.drain(..).collect();
                assert!(all_combinations.is_empty());
                for mut s in started {
                    for c in VALID_CARD2 {
                        s[i] = c;
                        all_combinations.push(s.clone());
                    }
                }
            }

            let set = all_combinations.iter().map(Set::from_cards).max().unwrap();
            Hand { cards, bid, set }
        })
        .collect()
}

fn part_one(hands: &Vec<Hand<Card>>) -> usize {
    let mut sorted = hands.clone();
    sorted.sort_unstable();
    sorted
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid)
        .sum()
}

fn part_two(hands: &Vec<Hand<Card2>>) -> usize {
    let mut sorted = hands.clone();
    sorted.sort_unstable();
    sorted
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid)
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = get_input("2023", "7");
    let hands = parse(&input);

    let now = time::Instant::now();
    let sol_p1 = part_one(&hands);
    println!(
        "Solution part one: {sol_p1} took: {}μs",
        now.elapsed().as_micros()
    );

    let hands = parse2(&input);
    let now = time::Instant::now();
    let sol_p2 = part_two(&hands);
    println!(
        "Solution part two: {sol_p2} took: {}μs",
        now.elapsed().as_micros()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_parse() {
        use Card::*;
        let hands = parse(TEST);

        let m1 = Set::OnePair(Card::N2.into());
        let m2 = Set::OnePair(Card::N3.into());

        assert_eq!(m1.cmp(&m2), Ordering::Equal);

        assert_eq!(hands[0].bid, 765);
        assert_eq!(hands[1].set, Set::ThreeOAK(N5.into()));
        assert!(hands[2] > hands[3]);
        assert_eq!(hands[2].cards, vec![K, K, N6, N7, N7]);
    }

    #[test]
    fn test_part_one() {
        let hands = parse(TEST);
        assert_eq!(part_one(&hands), 6440);
    }

    #[test]
    fn test_part_two() {
        let hands = parse2(TEST);
        assert_eq!(part_two(&hands), 5905);
    }
}
