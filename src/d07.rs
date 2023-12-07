use crate::harness::Harness;

pub fn run(h: &mut Harness) {
    h.begin(7)
        //.input_override("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483\n")
        //.input_override("JJJJJ 1\n")
        .run_part(1, |text| {
            let mut scores = text
                .lines()
                .map(|line| {
                    let hand = Hand::parse(line, false).unwrap();
                    (hand.score(false), hand.bid)
                })
                .collect::<Vec<_>>();
            scores.sort_unstable_by_key(|&(score, _)| score);
            scores
                .iter()
                .enumerate()
                .map(|(i, (_, bid))| (i + 1) as u32 * bid)
                .sum::<u32>()
        })
        .run_part(2, |text| {
            let mut scores = text
                .lines()
                .map(|line| {
                    let hand = Hand::parse(line, true).unwrap();
                    (hand.score(true), hand.bid)
                })
                .collect::<Vec<_>>();
            scores.sort_unstable_by_key(|&(score, _)| score);
            scores
                .iter()
                .enumerate()
                .map(|(i, (_, bid))| (i + 1) as u32 * bid)
                .sum::<u32>()
        });
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Hand {
    cards_bits: u32,
    bid: u32,
}

impl Hand {
    fn score(&self, part2: bool) -> u32 {
        //let mut equal_counts = [0; 5];
        //for i in 1..5 {
        //    let shifted = self.cards >> (i * 4);
        //    println!("{:08X}", self.cards);
        //    println!("{shifted:08X}");
        //    println!();
        //    for j in 0..5 - i {
        //        let mask = 0xF << (j * 4);
        //        if self.cards & mask == shifted & mask {
        //            equal_counts[i as usize] += 1;
        //        }
        //    }
        //}

        // We have 3 unused elements to eliminate bounds checking
        let mut counts = [0; 16];
        for i in 0..5 {
            let card = (self.cards_bits >> (i * 4)) & 0xF;
            counts[card as usize] += 1;
        }

        if part2 {
            let jokers = counts[0];
            counts[0] = 0;
            counts.sort_unstable();
            counts[15] += jokers;
        } else {
            counts.sort_unstable();
        }

        let level = if counts[15] == 5 {
            // Five of a kind
            6
        } else if counts[15] == 4 {
            // Four of a kind
            5
        } else if counts[15] == 3 && counts[14] == 2 {
            // Full house
            4
        } else if counts[15] == 3 {
            // Three of a kind
            3
        } else if counts[15] == 2 && counts[14] == 2 {
            // Two pair
            2
        } else if counts[15] == 2 {
            // Pair
            1
        } else {
            // High card
            0
        };

        self.cards_bits | (level << 20)
    }

    fn parse(s: &str, part2: bool) -> Result<Self, ()> {
        let (raw_cards, bid) = s.split_once(' ').ok_or(())?;
        let bid = bid.parse::<u32>().map_err(|_| ())?;

        if raw_cards.as_bytes().len() != 5 {
            return Err(());
        }
        let mut cards_bits = 0;
        for (i, &c) in raw_cards.as_bytes().iter().enumerate() {
            let digit = c.wrapping_sub(b'2');
            let v;
            if digit <= 7 {
                v = digit + part2 as u8;
            } else if c == b'T' {
                v = 8 + part2 as u8;
            } else if c == b'J' {
                v = if part2 { 0 } else { 9 };
            } else if c == b'Q' {
                v = 10;
            } else if c == b'K' {
                v = 11;
            } else if c == b'A' {
                v = 12;
            } else {
                return Err(());
            }

            // Put the first card in the most significant position
            cards_bits |= u32::from(v) << ((4 - i) * 4);
        }

        Ok(Self { cards_bits, bid })
    }
}
