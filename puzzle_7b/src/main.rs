use counter::Counter;
use std::cmp::Ordering;

const FIVE_OF_A_KIND: u16 = 6;
const FOUR_OF_A_KIND: u16 = 5;
const FULL_HOUSE: u16 = 4;
const THREE_OF_A_KIND: u16 = 3;
const TWO_PAIR: u16 = 2;
const ONE_PAIR: u16= 1;
const HIGH_CARD: u16 = 0;

// 'J' is now the lowest card value
const CARD_VALUES: [char; 13] = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

struct Hand {
    cards: Vec<char>,
    strength: u32,
    score: u16
}

fn main() {
    let lines = include_str!("../input.txt").lines().map(|l| l.split(' ').collect::<Vec<&str>>());
    let mut total_scores: Vec<Hand> = Vec::new();

    for line in lines {
        let hand: &str = line[0];
        let strength: u32 = line[1].parse().unwrap();
        
        // Count occurences of each card, and then count the occurences of each count
        let num_jokers = hand.chars().filter(|&c| c == 'J').count();
        let counts = hand.chars().filter(|&c| c != 'J').collect::<Counter<char>>();
        let counts_counts = counts.values().collect::<Counter<_>>();               

        let mut full_hand = Hand {
            cards: hand.chars().collect(),
            strength: strength,
            score: 0
        };
        
        // Determine the highest count
        let max_count = match counts_counts.keys().max() {
            Some(num) => **num,
            None => 0,
        };

        // Determine the score of the hand
        // Dislike this, but it works
        if max_count + num_jokers >= 5 {
            full_hand.score = FIVE_OF_A_KIND;
        }
        else if max_count + num_jokers == 4 {
            full_hand.score = FOUR_OF_A_KIND;
        }
        else if (counts_counts[&3] == 1 && counts_counts[&2] == 1) ||
        (counts_counts[&2] == 2 && num_jokers == 1) || 
        (counts_counts[&2] == 1 && counts_counts[&1] > 1 && num_jokers > 1) ||
        (counts_counts[&1] >= 2 && num_jokers > 2) {
            full_hand.score = FULL_HOUSE;
        }
        else if max_count + num_jokers == 3 {
            full_hand.score = THREE_OF_A_KIND;
        }
        else if counts_counts[&2] == 2 || (counts_counts[&2] == 1 && num_jokers > 0)
        || (counts_counts[&1] >= 2 && num_jokers > 1) {
            full_hand.score = TWO_PAIR;
        }
        else if counts_counts[&2] == 1 || (counts_counts[&1] >= 1 && num_jokers == 1) {
            full_hand.score = ONE_PAIR;
        }
        else {
            full_hand.score = HIGH_CARD;
        }

        println!("{} {} {:?}", full_hand.score, num_jokers, full_hand.cards);

        total_scores.push(full_hand);
    }

    // Sort by score, then by card values
    total_scores.sort_by(|a, b| sort_hands(a, b));
    let mut total_winnings: u32 = 0;
    for (i, hand) in total_scores.iter().enumerate() {
        let winning = hand.strength * (i as u32 + 1);
        total_winnings += winning;
    }

    println!("Total Winnings: {}", total_winnings);

}

fn sort_hands(hand_1: &Hand, hand_2: &Hand) -> Ordering {

    // Sort by score, then by card values
    if hand_1.score > hand_2.score {
        return Ordering::Greater;
    }
    else if hand_1.score < hand_2.score {
        return Ordering::Less;
    }
    else {
        for i in 0..hand_1.cards.len() {
            if CARD_VALUES.iter().position(|&r| r == hand_1.cards[i]) > CARD_VALUES.iter().position(|&r| r == hand_2.cards[i]) {
                return Ordering::Greater;
            }
            else if CARD_VALUES.iter().position(|&r| r == hand_1.cards[i]) < CARD_VALUES.iter().position(|&r| r == hand_2.cards[i]) {
                return Ordering::Less;
            }
        }   
    }
    return Ordering::Equal;
}
