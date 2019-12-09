use std::collections::HashMap;

pub trait Countable<A> {
    fn counts(self) -> HashMap<A, usize>;
}

impl<A, I> Countable<A> for I
where
    A: Eq,
    A: std::hash::Hash,
    I: Iterator<Item = A>,
{
    fn counts(self) -> HashMap<A, usize> {
        self.fold(HashMap::new(), |mut result, item| {
            let count = result.remove(&item).unwrap_or(0) + 1;
            result.insert(item, count);
            result
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Countable;
    use std::collections::HashMap;

    #[test]
    fn count_empty_is_empty() {
        assert_eq!(Vec::<i32>::new().into_iter().counts(), HashMap::new());
    }

    #[test]
    fn count_one_is_one() {
        assert_eq!(
            vec![0].into_iter().counts(),
            vec![(0, 1)].into_iter().collect()
        );
    }

    #[test]
    fn count_many_is_different() {
        assert_eq!(
            vec![0, 1, 1, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 6]
                .into_iter()
                .counts(),
            vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 1), (6, 1)]
                .into_iter()
                .collect()
        );
    }
}
