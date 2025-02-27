use std::iter::FusedIterator;

pub struct PermutationIter<'a, T> {
    elements: &'a [T],
    indices: Vec<usize>,
    cycles: Vec<usize>,
    r: usize,
    consumed: bool,
}

impl<'a, T> PermutationIter<'a, T> {
    pub fn new(elements: &'a [T], r: usize) -> Self {
        assert!(r <= elements.len());

        let n = elements.len();
        let indices = (0..n).collect();
        let cycles = (0..r).map(|i| n - i).collect();

        Self {
            elements,
            indices,
            cycles,
            r,
            consumed: false,
        }
    }
}

impl<'a, T: Clone> Iterator for PermutationIter<'a, T> {
    type Item = (Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.consumed {
            return None;
        }

        let permutation = self
            .indices
            .iter()
            .take(self.r)
            .map(|&i| self.elements[i].clone())
            .collect::<Vec<_>>();

        let remaining = self
            .indices
            .iter()
            .skip(self.r)
            .map(|&i| self.elements[i].clone())
            .collect::<Vec<_>>();

        for i in (0..self.r).rev() {
            self.cycles[i] -= 1;
            if self.cycles[i] == 0 {
                let removed = self.indices.remove(i);
                self.indices.push(removed);
                self.cycles[i] = self.elements.len() - i;

                if i == 0 {
                    self.consumed = true;
                }
            } else {
                let j = self.elements.len() - self.cycles[i];
                self.indices.swap(i, j);
                return Some((permutation, remaining));
            }
        }

        self.consumed = true;
        Some((permutation, remaining))
    }
}

impl<'a, T: Clone> FusedIterator for PermutationIter<'a, T> {}

pub struct CombinationIter<'a, T> {
    elements: &'a [T],
    indices: Vec<usize>,
    k: usize,
    consumed: bool,
}

impl<'a, T> CombinationIter<'a, T> {
    pub fn new(elements: &'a [T], k: usize) -> Self {
        assert!(k <= elements.len());
        Self {
            elements,
            indices: (0..k).collect(),
            k,
            consumed: false,
        }
    }
}

impl<'a, T: Clone> Iterator for CombinationIter<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.consumed {
            return None
        }

        if self.k == 0 {
            self.consumed = true;
            return Some(vec![]);
        }

        let combination = self
            .indices
            .iter()
            .map(|&i| self.elements[i].clone())
            .collect::<Vec<_>>();

        let mut i = self.k;
        while i > 0 {
            i -= 1;
            if self.indices[i] != i + self.elements.len() - self.k {
                break;
            }
        }

        if self.indices[i] == i + self.elements.len() - self.k {
            self.consumed = true;
        } else {
            self.indices[i] += 1;
            for j in i + 1..self.k {
                self.indices[j] = self.indices[j - 1] + 1;
            }
        }

        Some(combination)
    }
}

impl<'a, T: Clone> FusedIterator for CombinationIter<'a, T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutation_iter_partial() {
        let elements = vec!['A', 'B', 'C'];
        let perms = PermutationIter::new(&elements, 2);
        let actual: Vec<(Vec<char>, Vec<char>)> = perms.collect();
        let expected = vec![
            (vec!['A', 'B'], vec!['C']),
            (vec!['A', 'C'], vec!['B']),
            (vec!['B', 'A'], vec!['C']),
            (vec!['B', 'C'], vec!['A']),
            (vec!['C', 'A'], vec!['B']),
            (vec!['C', 'B'], vec!['A']),
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn permutation_iter_full() {
        let elements = vec!['A', 'B', 'C'];
        let perms = PermutationIter::new(&elements, 3);
        let actual: Vec<(Vec<char>, Vec<char>)> = perms.collect();
        let expected = vec![
            (vec!['A', 'B', 'C'], vec![]),
            (vec!['A', 'C', 'B'], vec![]),
            (vec!['B', 'A', 'C'], vec![]),
            (vec!['B', 'C', 'A'], vec![]),
            (vec!['C', 'A', 'B'], vec![]),
            (vec!['C', 'B', 'A'], vec![]),
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn permutation_iter_single() {
        let elements = vec!['A', 'B', 'C'];
        let perms = PermutationIter::new(&elements, 1);
        let actual: Vec<(Vec<char>, Vec<char>)> = perms.collect();
        let expected = vec![
            (vec!['A'], vec!['B', 'C']),
            (vec!['B'], vec!['A', 'C']),
            (vec!['C'], vec!['A', 'B']),
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn permutation_iter_zero() {
        let elements = vec!['A', 'B', 'C'];
        let perms = PermutationIter::new(&elements, 0);
        let actual: Vec<(Vec<char>, Vec<char>)> = perms.collect();
        let expected = vec![
            (vec![], vec!['A', 'B', 'C']),
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn combination_iter_full() {
        let elements = vec!['A', 'B', 'C', 'D'];
        let combs = CombinationIter::new(&elements, 4);
        let actual: Vec<Vec<char>> = combs.collect();
        let expected = vec![vec!['A', 'B', 'C', 'D']];

        assert_eq!(actual, expected);
    }

    #[test]
    fn combination_iter_partial() {
        let elements = vec!['A', 'B', 'C', 'D'];
        let combs = CombinationIter::new(&elements, 2);
        let actual: Vec<Vec<char>> = combs.collect();
        let expected = vec![
            vec!['A', 'B'],
            vec!['A', 'C'],
            vec!['A', 'D'],
            vec!['B', 'C'],
            vec!['B', 'D'],
            vec!['C', 'D'],
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn combination_iter_zero() {
        let elements = vec!['A', 'B', 'C', 'D'];
        let combs = CombinationIter::new(&elements, 0);
        let actual: Vec<Vec<char>> = combs.collect();
        let expected = vec![Vec::<char>::new()];

        assert_eq!(actual, expected);
    }
}
