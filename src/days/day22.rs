use crate::common::Solution;

#[derive(Debug)]
enum Deck {
    Initial(Vec<usize>),
    Stack(Box<Deck>),
    Cut(Box<Deck>, isize),
    Deal(Box<Deck>, usize),
}

impl Deck {
    fn len(&self) -> usize {
        match self {
            Self::Initial(cards) => cards.len(),
            Self::Stack(deck) => deck.len(),
            Self::Cut(deck, _) => deck.len(),
            Self::Deal(deck, _) => deck.len(),
        }
    }

    fn new(len: usize) -> Self {
        Self::Initial((0..len).collect())
    }

    fn stack(self) -> Self {
        Self::Stack(Box::new(self))
    }

    fn cut(self, n: isize) -> Self {
        Self::Cut(Box::new(self), n)
    }

    fn deal(self, n: usize) -> Self {
        Self::Deal(Box::new(self), n)
    }
}

impl std::ops::Index<usize> for Deck {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Self::Initial(cards) => &cards[index],
            Self::Stack(deck) => &deck[deck.len() - index - 1],
            Self::Cut(deck, n) => {
                &deck[(index as isize + n + self.len() as isize) as usize % self.len()]
            }
            Self::Deal(deck, n) => {
                for i in 0..self.len() {
                    if (i * n) % self.len() == index {
                        return &deck[i];
                    }
                }
                unreachable!()
            }
        }
    }
}

fn solve_a(lines: &[String]) -> usize {
    let mut deck: Deck = Deck::new(10007);

    for line in lines {
        if &line[0..3] == "cut" {
            deck = deck.cut(line[4..].parse().unwrap());
        } else if &line[0..9] == "deal with" {
            deck = deck.deal(line[20..].parse().unwrap());
        } else {
            deck = deck.stack();
        }
    }

    // let output: Vec<usize> = (0..deck.len()).map(|i| deck[i]).collect();
    // println!("{:?}", deck);
    // println!("{:?}", output);

    for i in 0..deck.len() {
        println!("{}", i);
        if deck[i] == 2019 {
            return i;
        }
    }

    unreachable!()
}

pub fn solve(lines: &[String]) -> Solution {
    let a_solution = solve_a(lines);
    // let b_solution = solve_b(&world);
    let b_solution = "";
    (a_solution.to_string(), b_solution.to_string())
}

#[cfg(test)]
mod tests {
    use super::Deck;

    #[test]
    fn stack() {
        let deck = Deck::new(10).stack();
        let output: Vec<usize> = (0..10).map(|i| deck[i]).collect();
        assert_eq!(output, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn cut() {
        let deck = Deck::new(10).cut(3);
        let output: Vec<usize> = (0..10).map(|i| deck[i]).collect();
        assert_eq!(output, vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn deal() {
        let deck = Deck::new(10).deal(3);
        let output: Vec<usize> = (0..10).map(|i| deck[i]).collect();
        assert_eq!(output, vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }

    #[test]
    fn example1() {
        let deck = Deck::new(10).deal(7).stack().stack();
        let output: Vec<usize> = (0..10).map(|i| deck[i]).collect();
        assert_eq!(output, vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn example2() {
        let deck = Deck::new(10).cut(6).deal(7).stack();
        let output: Vec<usize> = (0..10).map(|i| deck[i]).collect();
        assert_eq!(output, vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn example3() {
        let deck = Deck::new(10).deal(7).deal(9).cut(-2);
        let output: Vec<usize> = (0..10).map(|i| deck[i]).collect();
        assert_eq!(output, vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn example4() {
        let deck = Deck::new(10)
            .stack()
            .cut(-2)
            .deal(7)
            .cut(8)
            .cut(-4)
            .deal(7)
            .cut(3)
            .deal(9)
            .deal(3)
            .cut(-1);
        let output: Vec<usize> = (0..10).map(|i| deck[i]).collect();
        assert_eq!(output, vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }

    #[test]
    fn deal13_1() {
        let deck = Deck::new(13).deal(1);
        let output: Vec<usize> = (0..deck.len()).map(|i| deck[i]).collect();
        assert_eq!(output, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }

    #[test]
    fn deal13_2() {
        let deck = Deck::new(13).deal(2);
        let output: Vec<usize> = (0..deck.len()).map(|i| deck[i]).collect();
        assert_eq!(output, vec![0, 7, 1, 8, 2, 9, 3, 10, 4, 11, 5, 12, 6]);
    }

    #[test]
    fn deal13_3() {
        let deck = Deck::new(13).deal(3);
        let output: Vec<usize> = (0..deck.len()).map(|i| deck[i]).collect();
        assert_eq!(output, vec![0, 9, 5, 1, 10, 6, 2, 11, 7, 3, 12, 8, 4]);
    }

    #[test]
    fn deal13_4() {
        let deck = Deck::new(13).deal(4);
        let output: Vec<usize> = (0..deck.len()).map(|i| deck[i]).collect();
        assert_eq!(output, vec![0, 10, 7, 4, 1, 11, 8, 5, 2, 12, 9, 6, 3]);
    }

    #[test]
    fn deal13_5() {
        let deck = Deck::new(13).deal(5);
        let output: Vec<usize> = (0..deck.len()).map(|i| deck[i]).collect();
        assert_eq!(output, vec![0, 8, 3, 11, 6, 1, 9, 4, 12, 7, 2, 10, 5]);
    }

    #[test]
    fn deal13_6() {
        let deck = Deck::new(13).deal(6);
        let output: Vec<usize> = (0..deck.len()).map(|i| deck[i]).collect();
        assert_eq!(output, vec![0, 11, 9, 7, 5, 3, 1, 12, 10, 8, 6, 4, 2]);
    }

    #[test]
    fn deal13_7() {
        let deck = Deck::new(13).deal(7);
        let output: Vec<usize> = (0..deck.len()).map(|i| deck[i]).collect();
        assert_eq!(output, vec![0, 2, 4, 6, 8, 10, 12, 1, 3, 5, 7, 9, 11]);
    }

    #[test]
    fn deal13_8() {
        let deck = Deck::new(13).deal(8);
        let output: Vec<usize> = (0..deck.len()).map(|i| deck[i]).collect();
        assert_eq!(output, vec![0, 5, 10, 2, 7, 12, 4, 9, 1, 6, 11, 3, 8]);
    }

    #[test]
    fn deal13_9() {
        let deck = Deck::new(13).deal(9);
        let output: Vec<usize> = (0..deck.len()).map(|i| deck[i]).collect();
        assert_eq!(output, vec![0, 3, 6, 9, 12, 2, 5, 8, 11, 1, 4, 7, 10]);
    }

    #[test]
    fn deal13_10() {
        let deck = Deck::new(13).deal(10);
        let output: Vec<usize> = (0..deck.len()).map(|i| deck[i]).collect();
        assert_eq!(output, vec![0, 4, 8, 12, 3, 7, 11, 2, 6, 10, 1, 5, 9]);
    }

    #[test]
    fn deal13_11() {
        let deck = Deck::new(13).deal(11);
        let output: Vec<usize> = (0..deck.len()).map(|i| deck[i]).collect();
        assert_eq!(output, vec![0, 6, 12, 5, 11, 4, 10, 3, 9, 2, 8, 1, 7]);
    }

    #[test]
    fn deal13_12() {
        let deck = Deck::new(13).deal(12);
        let output: Vec<usize> = (0..deck.len()).map(|i| deck[i]).collect();
        assert_eq!(output, vec![0, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }
}
