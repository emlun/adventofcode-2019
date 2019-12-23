use crate::common::Solution;

#[derive(Debug)]
enum Deck {
    Initial(u128),
    Stack(Box<Deck>, u128),
    Cut(Box<Deck>, u128, u128),
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

#[derive(Clone, Debug)]
struct ModPolynomial {
    k: Vec<u128>,
    modulus: u128,
}

impl ModPolynomial {
    fn apply(&self, x: u128) -> u128 {
        self.k.iter().enumerate().fold(0, |sum, (i, k)| {
            let part = k * modpow(x, i as u128, self.modulus) % self.modulus;
            (sum + part) % self.modulus
        })
    }

    fn compose_deg1(&self, other: &ModPolynomial) -> ModPolynomial {
        assert_eq!(self.k.len(), 2);
        assert_eq!(other.k.len(), 2);
        assert_eq!(self.modulus, other.modulus);
        ModPolynomial {
            k: vec![
                (self.k[0] + self.k[1] * other.k[0]) % self.modulus,
                (self.k[1] * other.k[1]) % self.modulus,
            ],
            modulus: self.modulus,
        }
    }

    fn self_composed_deg1(&self, mut times: u128) -> ModPolynomial {
        assert_eq!(self.k.len(), 2);

        let mut composed = ModPolynomial {
            k: vec![0, 1],
            modulus: self.modulus,
        };
        let mut self_pow2 = self.clone();

        while times > 0 {
            if times % 2 == 1 {
                composed = composed.compose_deg1(&self_pow2);
            }
            self_pow2 = self_pow2.compose_deg1(&self_pow2);
            times >>= 1;
        }

        composed
    }
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
        Self::Cut(Box::new(self), (l as i128 + n) as u128 % l, l)
    }

    fn deal(self, n: u128) -> Self {
        let l = self.len();
        let modulus = self.len();
        let ninv = modinv(n, modulus);
        Self::Deal(Box::new(self), n, ninv, l)
    }

    fn get(&self, index: u128) -> u128 {
        match self {
            Self::Initial(_) => index,
            Self::Stack(deck, len) => deck.get(len - index - 1),
            Self::Cut(deck, n, len) => deck.get((index + n) % len),
            Self::Deal(deck, _, ninv, len) => deck.get((index * ninv) % len),
        }
    }

    fn simplify(&self) -> ModPolynomial {
        match self {
            Self::Initial(len) => ModPolynomial {
                k: vec![0, 1],
                modulus: *len,
            },
            Self::Stack(deck, len) => {
                let me = ModPolynomial {
                    k: vec![len - 1, len - 1],
                    modulus: *len,
                };
                deck.simplify().compose_deg1(&me)
            }
            Self::Cut(deck, n, len) => {
                let me = ModPolynomial {
                    k: vec![*n, 1],
                    modulus: *len,
                };
                deck.simplify().compose_deg1(&me)
            }
            Self::Deal(deck, _, ninv, len) => {
                let me = ModPolynomial {
                    k: vec![0, *ninv],
                    modulus: *len,
                };
                deck.simplify().compose_deg1(&me)
            }
        }
    }

    fn get_repeated(&self, index: u128, depth: u128) -> u128 {
        if depth == 0 {
            index
        } else {
            self.get_repeated(self.get(index), depth - 1)
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

    for depth in 0..100 {
        let current = once_deck.get_repeated(2020, depth);
        println!("{} {}", depth, current);
    }

    println!();

    let poly: ModPolynomial = once_deck.simplify();
    let mut prev = 2020;
    for depth in 1..100 {
        let current = once_deck.get(prev);
        let polyout = poly.apply(prev);
        // d += depth * depth * depth;
        prev = current;
        // if depth % 1_000_000 == 0 {
        println!(
            "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
            depth,
            current,
            polyout,
            poly.self_composed_deg1(1).apply(prev),
            poly.self_composed_deg1(2).apply(prev),
            poly.self_composed_deg1(3).apply(prev),
            poly.self_composed_deg1(4).apply(prev),
            poly.self_composed_deg1(5).apply(prev),
            poly.self_composed_deg1(6).apply(prev),
            poly.self_composed_deg1(7).apply(prev),
            poly.self_composed_deg1(8).apply(prev),
            poly.self_composed_deg1(9).apply(prev),
            poly.self_composed_deg1(10).apply(prev),
            poly.self_composed_deg1(11).apply(prev),
            poly.self_composed_deg1(12).apply(prev),
            poly.self_composed_deg1(13).apply(prev),
            poly.self_composed_deg1(14).apply(prev),
            poly.self_composed_deg1(15).apply(prev),
            poly.self_composed_deg1(16).apply(prev),
        );
        // }
        // assert_eq!(anti, prev);
        // multi_deck = shuffle(lines, multi_deck);
    }

    let mut multi_deck: Deck = Deck::new(119315717514047);
    for i in 0..100 {
        assert_eq!(once_deck.get_repeated(2020, i), multi_deck.get(2020));
        assert_eq!(
            once_deck.get_repeated(2020, i),
            poly.self_composed_deg1(i).apply(2020)
        );
        assert_eq!(multi_deck.get(2020), poly.self_composed_deg1(i).apply(2020));
        multi_deck = shuffle(&lines, multi_deck);
    }

    poly.self_composed_deg1(101741582076661).apply(2020)
}

pub fn solve(lines: &[String]) -> Solution {
    let a_solution = solve_a(lines);
    let b_solution = solve_b(lines);
    (a_solution.to_string(), b_solution.to_string())
}

#[cfg(test)]
mod tests {
    use super::Deck;

    fn testit(deck: Deck, expected: Vec<u128>) {
        let poly = deck.simplify();
        let output: Vec<u128> = (0..deck.len()).map(|i| deck.get(i)).collect();
        let poly_output: Vec<u128> = (0..deck.len()).map(|i| poly.apply(i)).collect();
        assert_eq!(output, expected);
        assert_eq!(poly_output, expected);
    }

    #[test]
    fn stack() {
        testit(Deck::new(10).stack(), vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn stack_stack() {
        testit(
            Deck::new(13).stack().stack(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        );
    }

    #[test]
    fn cut() {
        testit(Deck::new(10).cut(3), vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn cut_cut() {
        testit(
            Deck::new(13).cut(3).cut(5),
            vec![8, 9, 10, 11, 12, 0, 1, 2, 3, 4, 5, 6, 7],
        );
    }

    #[test]
    fn deal_5_deal_3() {
        testit(
            Deck::new(13).deal(5).deal(3),
            vec![0, 7, 1, 8, 2, 9, 3, 10, 4, 11, 5, 12, 6],
        );
    }

    #[test]
    fn deal_5_cut_3() {
        testit(
            Deck::new(13).deal(5).cut(3),
            vec![11, 6, 1, 9, 4, 12, 7, 2, 10, 5, 0, 8, 3],
        );
    }

    #[test]
    fn cut_3_deal_5() {
        testit(
            Deck::new(13).cut(3).deal(5),
            vec![3, 11, 6, 1, 9, 4, 12, 7, 2, 10, 5, 0, 8],
        );
    }

    #[test]
    fn deal_5_stack() {
        testit(
            Deck::new(13).deal(5).stack(),
            vec![5, 10, 2, 7, 12, 4, 9, 1, 6, 11, 3, 8, 0],
        );
    }

    #[test]
    fn stack_deal_5() {
        testit(
            Deck::new(13).stack().deal(5),
            vec![12, 4, 9, 1, 6, 11, 3, 8, 0, 5, 10, 2, 7],
        );
    }

    #[test]
    fn stack_cut_5() {
        testit(
            Deck::new(13).stack().cut(5),
            vec![7, 6, 5, 4, 3, 2, 1, 0, 12, 11, 10, 9, 8],
        );
    }

    #[test]
    fn cut_5_stack() {
        testit(
            Deck::new(13).cut(5).stack(),
            vec![4, 3, 2, 1, 0, 12, 11, 10, 9, 8, 7, 6, 5],
        );
    }

    #[test]
    fn deal13_1() {
        testit(
            Deck::new(13).deal(1),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        );
    }

    #[test]
    fn deal13_2() {
        testit(
            Deck::new(13).deal(2),
            vec![0, 7, 1, 8, 2, 9, 3, 10, 4, 11, 5, 12, 6],
        );
    }

    #[test]
    fn deal13_3() {
        testit(
            Deck::new(13).deal(3),
            vec![0, 9, 5, 1, 10, 6, 2, 11, 7, 3, 12, 8, 4],
        );
    }

    #[test]
    fn deal13_4() {
        testit(
            Deck::new(13).deal(4),
            vec![0, 10, 7, 4, 1, 11, 8, 5, 2, 12, 9, 6, 3],
        );
    }

    #[test]
    fn deal13_5() {
        testit(
            Deck::new(13).deal(5),
            vec![0, 8, 3, 11, 6, 1, 9, 4, 12, 7, 2, 10, 5],
        );
    }

    #[test]
    fn deal13_6() {
        testit(
            Deck::new(13).deal(6),
            vec![0, 11, 9, 7, 5, 3, 1, 12, 10, 8, 6, 4, 2],
        );
    }

    #[test]
    fn deal13_7() {
        testit(
            Deck::new(13).deal(7),
            vec![0, 2, 4, 6, 8, 10, 12, 1, 3, 5, 7, 9, 11],
        );
    }

    #[test]
    fn deal13_8() {
        testit(
            Deck::new(13).deal(8),
            vec![0, 5, 10, 2, 7, 12, 4, 9, 1, 6, 11, 3, 8],
        );
    }

    #[test]
    fn deal13_9() {
        testit(
            Deck::new(13).deal(9),
            vec![0, 3, 6, 9, 12, 2, 5, 8, 11, 1, 4, 7, 10],
        );
    }

    #[test]
    fn deal13_10() {
        testit(
            Deck::new(13).deal(10),
            vec![0, 4, 8, 12, 3, 7, 11, 2, 6, 10, 1, 5, 9],
        );
    }

    #[test]
    fn deal13_11() {
        testit(
            Deck::new(13).deal(11),
            vec![0, 6, 12, 5, 11, 4, 10, 3, 9, 2, 8, 1, 7],
        );
    }

    #[test]
    fn deal13_12() {
        testit(
            Deck::new(13).deal(12),
            vec![0, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
        );
    }
}
