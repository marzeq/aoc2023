use std::collections::HashMap;

pub fn run(part: u8, input: String) {
    let mut hands: Vec<Vec<char>> = Vec::new();
    let mut bids: HashMap<Vec<char>, u64> = HashMap::new();

    input.lines().for_each(|line| {
        let mut words = line.split_whitespace();

        let hand = words.next().unwrap().chars().collect::<Vec<char>>();
        let bid = words.next().unwrap().parse::<u64>().unwrap();

        if hand.len() != 5 {
            panic!("Invalid hand: {}", line);
        }

        hands.push(hand.clone());
        bids.insert(hand, bid);
    });

    fn check_hand(hand: Vec<char>, part: u8) -> (Vec<char>, u8) {
        let mut hand = hand.clone();

        let jcount = if part == 2 {
            hand.iter().filter(|&&card| card == 'J').count()
        } else {
            0
        };

        if part == 2 {
            hand.retain(|&card| card != 'J');
        }

        // handle speciall all joker edge case
        if part == 2 && hand.len() == 0 {
            return (hand, 5);
        }

        let mut maxcard = '0';
        let mut maxcount = 0;

        hand.clone().iter().for_each(|&card| {
            let count = hand.iter().filter(|&&c| c == card).count() + jcount;

            if count > maxcount {
                maxcount = count;
                maxcard = card;
            }
        });

        hand.retain(|&card| card != maxcard);

        (hand, maxcount as u8)
    }

    fn is_five_of_a_kind(hand: Vec<char>, part: u8) -> bool {
        check_hand(hand, part).1 == 5
    }

    fn is_four_of_a_kind(hand: Vec<char>, part: u8) -> bool {
        check_hand(hand, part).1 == 4
    }

    fn is_three_of_a_kind(hand: Vec<char>, part: u8) -> bool {
        check_hand(hand, part).1 == 3
    }

    fn is_pair(hand: Vec<char>, part: u8) -> bool {
        check_hand(hand, part).1 == 2
    }

    fn is_full_house(hand: Vec<char>, part: u8) -> bool {
        let (matched, maxcount) = check_hand(hand, part);

        if maxcount != 3 {
            return false;
        }

        let (_, maxcount) = check_hand(matched, part);

        maxcount == 2
    }

    fn is_two_pair(hand: Vec<char>, part: u8) -> bool {
        let (matched, maxcount) = check_hand(hand, part);

        if maxcount != 2 {
            return false;
        }

        let (_, maxcount) = check_hand(matched, part);

        maxcount == 2
    }

    fn is_high_card(hand: Vec<char>, part: u8) -> bool {
        for i in 0..5 {
            if hand.iter().filter(|&&card| card == hand[i]).count() != 1 {
                return false;
            }
        }

        true
    }

    fn bigger_hand(hand1: Vec<char>, hand2: Vec<char>, part: u8) -> std::cmp::Ordering {
        let a = hand1[0];
        let b = hand2[0];

        let a_num = if a.is_digit(10) {
            a.to_digit(10).unwrap()
        } else {
            match a {
                'T' => 10,
                'J' => {
                    if part == 1 {
                        11
                    } else {
                        1
                    }
                }
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("Invalid card: {}", a),
            }
        };

        let b_num = if b.is_digit(10) {
            b.to_digit(10).unwrap()
        } else {
            match b {
                'T' => 10,
                'J' => {
                    if part == 1 {
                        11
                    } else {
                        1
                    }
                }
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("Invalid card: {}", b),
            }
        };

        if a_num > b_num {
            std::cmp::Ordering::Greater
        } else if a_num < b_num {
            std::cmp::Ordering::Less
        } else {
            // compare by second card
            bigger_hand(hand1[1..].to_vec(), hand2[1..].to_vec(), part)
        }
    }

    fn assign_power_rating(hand: Vec<char>, part: u8) -> u8 {
        if is_five_of_a_kind(hand.clone(), part) {
            7
        } else if is_four_of_a_kind(hand.clone(), part) {
            6
        } else if is_full_house(hand.clone(), part) {
            5
        } else if is_three_of_a_kind(hand.clone(), part) {
            4
        } else if is_two_pair(hand.clone(), part) {
            3
        } else if is_pair(hand.clone(), part) {
            2
        } else if is_high_card(hand.clone(), part) {
            1
        } else {
            0
        }
    }

    hands.sort_by(|a, b| {
        let a_1 = a.clone();
        let b_1 = b.clone();

        let a_power = assign_power_rating(a_1, part);
        let b_power = assign_power_rating(b_1, part);

        if a_power > b_power {
            return std::cmp::Ordering::Greater;
        } else if a_power < b_power {
            return std::cmp::Ordering::Less;
        } else {
            return bigger_hand(a.clone(), b.clone(), part);
        }
    });

    let sum = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            let bid = bids.get(hand).unwrap();

            bid * (i + 1) as u64
        })
        .sum::<u64>();

    println!("{sum}");
}
