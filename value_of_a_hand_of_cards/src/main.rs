enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
    Ace,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Hand { cards: vec![] }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        let mut finval: usize = 0;

        for card in &self.cards {
            let cardval: usize = match card {
                Card::Two => 2,
                Card::Three => 3,
                Card::Four => 4,
                Card::Five => 5,
                Card::Six => 6,
                Card::Seven => 7,
                Card::Eight => 8,
                Card::Nine => 9,
                Card::Jack | Card::Queen | Card::King => 10,
                Card::Ace => {
                    if finval > 21 {
                        1
                    } else {
                        11
                    }
                }
            };

            finval += cardval;
        }

        finval
    }

    fn is_loosing_hand(&self) -> bool {
        self.value() > 21
    }
}

fn main() {
    let mut hand_1 = Hand::new();
    hand_1.add(Card::King);
    hand_1.add(Card::Ace);
    println!("Is Hand_1 Loosing Hand? : {}", hand_1.is_loosing_hand());

    let mut hand_2 = Hand::new();
    hand_2.add(Card::Queen);
    hand_2.add(Card::Jack);
    println!("Is Hand_2 Loosing Hand? : {}", hand_2.is_loosing_hand());

    let mut hand_3 = Hand::new();
    hand_3.add(Card::Nine);
    hand_3.add(Card::Eight);
    hand_3.add(Card::Seven);
    println!("Is Hand_3 Loosing Hand? : {}", hand_3.is_loosing_hand());

    let mut hand_4 = Hand::new();
    hand_4.add(Card::Six);
    hand_4.add(Card::Five);
    hand_4.add(Card::Four);
    println!("Is Hand_4 Loosing Hand? : {}", hand_4.is_loosing_hand());

    let mut hand_5 = Hand::new();
    hand_5.add(Card::Two);
    hand_5.add(Card::Three);
    hand_5.add(Card::Ace);
    hand_5.add(Card::Queen);
    println!("Is Hand_5 Loosing Hand? : {}", hand_5.is_loosing_hand());
}

#[test]
fn test_empty_hand() {
    let hand = Hand::new();
    assert_eq!(hand.is_loosing_hand(), false);
}

#[test]
fn test_less_than_21() {
    let mut hand = Hand::new();
    hand.add(Card::Two);
    hand.add(Card::Seven);
    assert_eq!(hand.is_loosing_hand(), false);
}

#[test]
fn test_risky_hand() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Ace);
    assert_eq!(hand.is_loosing_hand(), false);
}
