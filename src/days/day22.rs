use crate::common::Solution;

#[derive(Debug)]
enum Deck {
    Initial(u128),
    Stack(Box<Deck>, u128),
    Cut(Box<Deck>, i128, u128),
    Deal(Box<Deck>, u128, u128, u128),
}

fn modinv(n: u128, modulus: u128) -> u128 {
    modpow(n, modulus - 2, modulus)
}

fn modpow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    let mut result = 1;
    base %= modulus;

    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus;
    }
    result
}

impl Deck {
    fn len(&self) -> u128 {
        match self {
            Self::Initial(len) => *len,
            Self::Stack(_, len) => *len,
            Self::Cut(_, _, len) => *len,
            Self::Deal(_, _, _, len) => *len,
        }
    }

    fn new(len: u128) -> Self {
        Self::Initial(len)
    }

    fn stack(self) -> Self {
        let l = self.len();
        Self::Stack(Box::new(self), l)
    }

    fn cut(self, n: i128) -> Self {
        let l = self.len();
        Self::Cut(Box::new(self), n, l)
    }

    fn deal(self, n: u128) -> Self {
        let l = self.len();
        let modulus = self.len();
        let ninv = modinv(n, modulus);
        Self::Deal(Box::new(self), n, ninv, l)
    }

    // fn simplify(self) -> Self {
    //     match self {
    //         Self::Initial(_) => self,
    //         Self::Stack(sub) => {
    //             if let Self::Stack(deck) = *sub {
    //                 deck.simplify()
    //             } else {
    //                 Self::Stack(Box::new(sub.simplify()))
    //             }
    //         }
    //         Self::Cut(sub, n1) => {
    //             if let Self::Cut(deck, n2) = *sub {
    //                 Self::Cut(Box::new(deck.simplify()), n1 + n2)
    //             } else {
    //                 Self::Cut(Box::new(sub.simplify()), n1)
    //             }
    //         }
    //         Self::Deal(sub, n, ninv) => Self::Deal(Box::new(sub.simplify()), n, ninv),
    //     }
    // }

    fn get(&self, index: u128) -> u128 {
        match self {
            Self::Initial(_) => index,
            Self::Stack(deck, len) => deck.get(len - index - 1),
            Self::Cut(deck, n, len) => deck.get((index as i128 + n + *len as i128) as u128 % len),
            Self::Deal(deck, _, ninv, len) => deck.get((index * ninv) % len),
        }
    }

    fn get_repeated(&self, index: u128, depth: u128) -> u128 {
        if depth == 0 {
            index
        } else {
            let i = match self {
                Self::Initial(_) => index,
                Self::Stack(deck, len) => deck.get(len - index - 1),
                Self::Cut(deck, n, len) => {
                    deck.get((index as i128 + *n + *len as i128) as u128 % len)
                }
                Self::Deal(deck, _, ninv, len) => deck.get((index * ninv) % len),
            };
            self.get_repeated(i, depth - 1)
        }
    }
}

fn shuffle(lines: &[String], mut deck: Deck) -> Deck {
    for line in lines {
        if &line[0..3] == "cut" {
            deck = deck.cut(line[4..].parse().unwrap());
        } else if &line[0..9] == "deal with" {
            deck = deck.deal(line[20..].parse().unwrap());
        } else {
            deck = deck.stack();
        }
    }

    deck
}

fn solve_a(lines: &[String]) -> u128 {
    let deck: Deck = shuffle(lines, Deck::new(10007));

    for i in 0..deck.len() {
        if deck.get(i) == 2019 {
            return i;
        }
    }

    unreachable!()
}

fn solve_b(lines: &[String]) -> u128 {
    let once_deck: Deck = shuffle(lines, Deck::new(119315717514047));

    let mut shuffle_times: u128 = 101741582076661;

    for depth in 0..100 {
        let current = once_deck.get_repeated(2020, depth);
        println!("{} {}", depth, current);
    }

    println!(
        "{} {}",
        2,
        once_deck.get_repeated(once_deck.get_repeated(2020, 4), 4)
    );

    let mut multi_deck: Deck = Deck::new(119315717514047);
    let mut prev = 2020;
    let mut d: u128 = 0;
    multi_deck = shuffle(lines, multi_deck);
    for depth in 0..1000 {
        let current = multi_deck.get_repeated(prev, depth * depth * depth);
        d += depth * depth * depth;
        prev = current;
        println!("{} {} {}", depth, d, current);
        // multi_deck = shuffle(lines, multi_deck);
    }

    // multi_deck.get(2020)
    0
}

pub fn solve(lines: &[String]) -> Solution {
    let a_solution = solve_a(lines);
    let b_solution = solve_b(lines);
    (a_solution.to_string(), b_solution.to_string())
}

#[cfg(test)]
mod tests {
    use super::Deck;

    #[test]
    fn stack() {
        let deck = Deck::new(10).stack();
        let output: Vec<u128> = (0..10).map(|i| deck.get(i)).collect();
        assert_eq!(output, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn cut() {
        let deck = Deck::new(10).cut(3);
        let output: Vec<u128> = (0..10).map(|i| deck.get(i)).collect();
        assert_eq!(output, vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn deal13_1() {
        let deck = Deck::new(13).deal(1);
        let output: Vec<u128> = (0..deck.len()).map(|i| deck.get(i)).collect();
        assert_eq!(output, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }

    #[test]
    fn deal13_2() {
        let deck = Deck::new(13).deal(2);
        let output: Vec<u128> = (0..deck.len()).map(|i| deck.get(i)).collect();
        assert_eq!(output, vec![0, 7, 1, 8, 2, 9, 3, 10, 4, 11, 5, 12, 6]);
    }

    #[test]
    fn deal13_3() {
        let deck = Deck::new(13).deal(3);
        let output: Vec<u128> = (0..deck.len()).map(|i| deck.get(i)).collect();
        assert_eq!(output, vec![0, 9, 5, 1, 10, 6, 2, 11, 7, 3, 12, 8, 4]);
    }

    #[test]
    fn deal13_4() {
        let deck = Deck::new(13).deal(4);
        let output: Vec<u128> = (0..deck.len()).map(|i| deck.get(i)).collect();
        assert_eq!(output, vec![0, 10, 7, 4, 1, 11, 8, 5, 2, 12, 9, 6, 3]);
    }

    #[test]
    fn deal13_5() {
        let deck = Deck::new(13).deal(5);
        let output: Vec<u128> = (0..deck.len()).map(|i| deck.get(i)).collect();
        assert_eq!(output, vec![0, 8, 3, 11, 6, 1, 9, 4, 12, 7, 2, 10, 5]);
    }

    #[test]
    fn deal13_6() {
        let deck = Deck::new(13).deal(6);
        let output: Vec<u128> = (0..deck.len()).map(|i| deck.get(i)).collect();
        assert_eq!(output, vec![0, 11, 9, 7, 5, 3, 1, 12, 10, 8, 6, 4, 2]);
    }

    #[test]
    fn deal13_7() {
        let deck = Deck::new(13).deal(7);
        let output: Vec<u128> = (0..deck.len()).map(|i| deck.get(i)).collect();
        assert_eq!(output, vec![0, 2, 4, 6, 8, 10, 12, 1, 3, 5, 7, 9, 11]);
    }

    #[test]
    fn deal13_8() {
        let deck = Deck::new(13).deal(8);
        let output: Vec<u128> = (0..deck.len()).map(|i| deck.get(i)).collect();
        assert_eq!(output, vec![0, 5, 10, 2, 7, 12, 4, 9, 1, 6, 11, 3, 8]);
    }

    #[test]
    fn deal13_9() {
        let deck = Deck::new(13).deal(9);
        let output: Vec<u128> = (0..deck.len()).map(|i| deck.get(i)).collect();
        assert_eq!(output, vec![0, 3, 6, 9, 12, 2, 5, 8, 11, 1, 4, 7, 10]);
    }

    #[test]
    fn deal13_10() {
        let deck = Deck::new(13).deal(10);
        let output: Vec<u128> = (0..deck.len()).map(|i| deck.get(i)).collect();
        assert_eq!(output, vec![0, 4, 8, 12, 3, 7, 11, 2, 6, 10, 1, 5, 9]);
    }

    #[test]
    fn deal13_11() {
        let deck = Deck::new(13).deal(11);
        let output: Vec<u128> = (0..deck.len()).map(|i| deck.get(i)).collect();
        assert_eq!(output, vec![0, 6, 12, 5, 11, 4, 10, 3, 9, 2, 8, 1, 7]);
    }

    #[test]
    fn deal13_12() {
        let deck = Deck::new(13).deal(12);
        let output: Vec<u128> = (0..deck.len()).map(|i| deck.get(i)).collect();
        assert_eq!(output, vec![0, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }
}
