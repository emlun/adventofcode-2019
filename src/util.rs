pub mod iter;

#[derive(Debug)]
pub struct Permutations<A> {
    items: Vec<A>,
    index_permutations: Option<Box<Permutations<usize>>>,
    current_index_permutation: Option<Vec<usize>>,
    current_insert_index: usize,
}

impl<A, I> From<I> for Permutations<A>
where
    A: Copy,
    I: IntoIterator<Item = A>,
{
    fn from(items: I) -> Permutations<A> {
        let items: Vec<A> = items.into_iter().collect();

        let mut index_permutations = if items.is_empty() {
            None
        } else {
            Some(Permutations::from(0..(items.len() - 1)))
        };

        let first_index_perm = if items.len() == 1 {
            Some(vec![0])
        } else {
            index_permutations.as_mut().and_then(|p| p.next())
        };

        Permutations {
            items,
            index_permutations: index_permutations.map(Box::new),
            current_index_permutation: first_index_perm,
            current_insert_index: 0,
        }
    }
}

impl<A> Iterator for Permutations<A>
where
    A: Copy,
{
    type Item = Vec<A>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_insert_index >= self.items.len() {
            if let Some(idx_perms) = self.index_permutations.as_mut() {
                self.current_index_permutation = idx_perms.next();
                self.current_insert_index = 0;
            }
        }

        if let Some(idx_perm) = self.current_index_permutation.as_ref() {
            let insert_index = self.current_insert_index;
            self.current_insert_index += 1;
            Some(
                (0..self.items.len())
                    .map(|i| {
                        if i < insert_index {
                            self.items[idx_perm[i] + 1]
                        } else if i == insert_index {
                            self.items[0]
                        } else {
                            self.items[idx_perm[i - 1] + 1]
                        }
                    })
                    .collect(),
            )
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::iter::Countable;
    use super::Permutations;
    use std::collections::HashMap;
    use std::collections::HashSet;

    #[test]
    fn empty_has_no_permutations() {
        assert_eq!(
            Permutations::from(vec![]).collect::<Vec<Vec<i32>>>(),
            Vec::<Vec<i32>>::new()
        );
    }

    #[test]
    fn one_has_one_permutation() {
        assert_eq!(
            Permutations::from(vec![42]).collect::<Vec<Vec<i32>>>(),
            vec![vec![42]]
        );
    }

    #[test]
    fn two_has_two_permutations() {
        assert_eq!(
            Permutations::from(vec![42, 1337]).collect::<Vec<Vec<i32>>>(),
            vec![vec![42, 1337], vec![1337, 42]]
        );
    }

    #[test]
    fn three_has_six_permutations() {
        assert_eq!(
            Permutations::from(vec![42, 1337, 4711]).collect::<Vec<Vec<i32>>>(),
            vec![
                vec![42, 1337, 4711],
                vec![1337, 42, 4711],
                vec![1337, 4711, 42],
                vec![42, 4711, 1337],
                vec![4711, 42, 1337],
                vec![4711, 1337, 42]
            ]
        );
    }

    #[test]
    fn five_has_120_permutations() {
        let perms: HashSet<Vec<i32>> = Permutations::from(vec![1, 2, 3, 4, 5]).collect();
        assert_eq!(perms.len(), 1 * 2 * 3 * 4 * 5);
        for v in perms {
            let vset: HashSet<&i32> = v.iter().collect();
            assert_eq!(vset.len(), 5);
        }
    }

    #[test]
    fn eight_has_lots_of_permutations() {
        let perms: HashSet<Vec<i32>> = Permutations::from(1..=8).collect();
        assert_eq!(perms.len(), 1 * 2 * 3 * 4 * 5 * 6 * 7 * 8);
        for v in perms {
            let vset: HashSet<&i32> = v.iter().collect();
            assert_eq!(vset.len(), 8);
        }
    }

    #[test]
    fn six_non_unique_has_duplicate_permutations() {
        let perms: HashSet<Vec<i32>> = Permutations::from((1..=3).chain(2..=4)).collect();
        assert_eq!(perms.len(), 1 * 2 * 3 * 4 * 5 * 6 / (2 * 2));
        for v in perms {
            assert_eq!(
                v.iter().counts(),
                vec![(&1, 1), (&2, 2), (&3, 2), (&4, 1)]
                    .into_iter()
                    .collect::<HashMap<&i32, usize>>()
            );
        }
    }

    #[test]
    fn can_permute_references() {
        let items: Vec<i32> = vec![42, 1337];
        assert_eq!(
            Permutations::from(items.iter()).collect::<Vec<Vec<&i32>>>(),
            vec![vec![&42, &1337], vec![&1337, &42]]
        );
    }

    #[test]
    fn can_permute_references_to_funny_types() {
        #[derive(Debug, Eq, PartialEq)]
        struct Thing<'a> {
            message: &'a str,
        }
        let items: Vec<Thing> = vec![Thing { message: "foo" }, Thing { message: "bar" }];
        let permutations: Vec<Vec<&Thing>> = Permutations::from(items.iter()).collect();
        assert_eq!(
            permutations,
            vec![vec![&items[0], &items[1]], vec![&items[1], &items[0]]]
        );
        let moved_items = items;
        assert_eq!(moved_items, moved_items);
    }
}
