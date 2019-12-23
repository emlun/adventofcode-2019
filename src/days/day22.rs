use crate::common::Solution;

fn modinv(n: u128, prime_modulus: u128) -> u128 {
    modpow(n, prime_modulus - 2, prime_modulus)
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

#[derive(Debug)]
enum Deck {
    Initial(u128),
    Stack(Box<Deck>, u128),
    Cut(Box<Deck>, u128, u128),
    Deal(Box<Deck>, u128, u128),
}

impl Deck {
    fn len(&self) -> u128 {
        match self {
            Self::Initial(len) => *len,
            Self::Stack(_, len) => *len,
            Self::Cut(_, _, len) => *len,
            Self::Deal(_, _, len) => *len,
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
        Self::Deal(Box::new(self), ninv, l)
    }

    #[allow(dead_code)]
    fn get(&self, index: u128) -> u128 {
        match self {
            Self::Initial(_) => index,
            Self::Stack(deck, len) => deck.get(len - index - 1),
            Self::Cut(deck, n, len) => deck.get((index + n) % len),
            Self::Deal(deck, ninv, len) => deck.get((index * ninv) % len),
        }
    }

    fn shuffle(self, lines: &[String]) -> Deck {
        let mut deck = self;
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
            Self::Deal(deck, ninv, len) => {
                let me = ModPolynomial {
                    k: vec![0, *ninv],
                    modulus: *len,
                };
                deck.simplify().compose_deg1(&me)
            }
        }
    }
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

    fn invert(&self) -> ModPolynomial {
        let kinv = modinv(self.k[1], self.modulus);
        ModPolynomial {
            k: vec![((self.modulus - self.k[0]) * kinv) % self.modulus, kinv],
            modulus: self.modulus,
        }
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

fn solve_a(lines: &[String]) -> u128 {
    Deck::new(10007)
        .shuffle(lines)
        .simplify()
        .invert()
        .apply(2019)
}

#[allow(clippy::unreadable_literal)]
fn solve_b(lines: &[String]) -> u128 {
    let deck: Deck = Deck::new(119315717514047).shuffle(lines);
    let poly: ModPolynomial = deck.simplify();
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
    use super::ModPolynomial;

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

    #[test]
    fn repeated_polynomial() {
        let lines: Vec<String> = [
            "cut -7812",
            "deal with increment 55",
            "cut -3909",
            "deal with increment 51",
            "deal into new stack",
            "deal with increment 4",
            "cut -77",
            "deal with increment 26",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let mut deck: Deck = Deck::new(119315717514047);
        let poly: ModPolynomial = Deck::new(119315717514047).shuffle(&lines).simplify();
        let init = 2020;

        for i in 0..100 {
            assert_eq!(deck.get(init), poly.self_composed_deg1(i).apply(init));
            deck = deck.shuffle(&lines);
        }
    }
}
