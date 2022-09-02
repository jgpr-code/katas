fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Hand {
    HighCard(HighCard),
    Pair(Pair),
    TwoPairs(TwoPairs),
    ThreeOfKind(ThreeOfKind),
    Straight(Straight),
    Flush(Flush),
    FullHouse(FullHouse),
    FourOfKind(FourOfKind),
    StraightFlush(StraightFlush),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct HighCard {
    card: u8,
    second_card: u8,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Pair {
    pair: u8,
    high_card: u8,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct TwoPairs {
    high: u8,
    low: u8,
    high_card: u8,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct ThreeOfKind {
    kind: u8,
    // TODO needed? high_card: u8
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Straight {
    end: u8,
    // TODO needed? high_card: u8
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Flush {
    // TODO needed? high_card: u8
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct FullHouse {
    three_kind: u8,
    // TODO needed? high_card: u8
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct FourOfKind {
    kind: u8,
    // TODO needed? high_card: u8
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct StraightFlush {
    end: u8,
    // TODO needed? high_card: u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn high_card_normal() {
        let winner = Hand::HighCard(HighCard {
            card: 9,
            second_card: 2,
        });
        let loser = Hand::HighCard(HighCard {
            card: 8,
            second_card: 7,
        });

        assert!(winner > loser);
    }

    #[test]
    fn high_card_tiebreaker_second_high_card() {
        let winner = Hand::HighCard(HighCard {
            card: 8,
            second_card: 7,
        });
        let loser = Hand::HighCard(HighCard {
            card: 8,
            second_card: 6,
        });

        assert!(winner > loser);
    }

    #[test]
    fn pair_normal() {
        let winner = Hand::Pair(Pair {
            pair: 9,
            high_card: 9,
        });
        let loser = Hand::Pair(Pair {
            pair: 8,
            high_card: 10,
        });

        assert!(winner > loser);
    }

    #[test]
    fn pair_tiebreaker_high_card() {
        let winner = Hand::Pair(Pair {
            pair: 8,
            high_card: 8,
        });
        let loser = Hand::Pair(Pair {
            pair: 8,
            high_card: 7,
        });

        assert!(winner > loser);
    }

    #[test]
    fn two_pairs_normal() {
        let winner = Hand::TwoPairs(TwoPairs {
            high: 9,
            low: 2,
            high_card: 9,
        });
        let loser = Hand::TwoPairs(TwoPairs {
            high: 8,
            low: 7,
            high_card: 10,
        });

        assert!(winner > loser);
    }

    #[test]
    fn two_pairs_tiebreaker_low_pair() {
        let winner = Hand::TwoPairs(TwoPairs {
            high: 8,
            low: 7,
            high_card: 10,
        });
        let loser = Hand::TwoPairs(TwoPairs {
            high: 8,
            low: 6,
            high_card: 11,
        });

        assert!(winner > loser);
    }

    #[test]
    fn two_pairs_tiebreaker_high_card() {
        let winner = Hand::TwoPairs(TwoPairs {
            high: 8,
            low: 7,
            high_card: 10,
        });
        let loser = Hand::TwoPairs(TwoPairs {
            high: 8,
            low: 7,
            high_card: 8,
        });

        assert!(winner > loser);
    }

    #[test]
    fn test_sort() {
        let expected = vec![
            Hand::HighCard(HighCard {
                card: 8,
                second_card: 6,
            }),
            Hand::HighCard(HighCard {
                card: 8,
                second_card: 7,
            }),
            Hand::HighCard(HighCard {
                card: 9,
                second_card: 2,
            }),
            Hand::Pair(Pair {
                pair: 8,
                high_card: 7,
            }),
            Hand::Pair(Pair {
                pair: 8,
                high_card: 8,
            }),
            Hand::TwoPairs(TwoPairs {
                high: 8,
                low: 6,
                high_card: 11,
            }),
            Hand::TwoPairs(TwoPairs {
                high: 8,
                low: 7,
                high_card: 8,
            }),
            Hand::TwoPairs(TwoPairs {
                high: 8,
                low: 7,
                high_card: 10,
            }),
        ];
        let mut vec = expected.clone();

        let mut rng = rand::thread_rng();
        vec.shuffle(&mut rng);

        assert_ne!(vec, expected);

        vec.sort();

        assert_eq!(vec, expected);
    }
}
