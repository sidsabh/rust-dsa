/// Local impl. of Heap's algorithm to generate permutatations of a set of size n
pub struct PermutationGenerator {
    size : usize,
    current_perm : Vec<usize>,
    counters: Vec<usize>,
    index: usize,
    first: bool
}

impl PermutationGenerator {

    pub fn new(n: usize) -> PermutationGenerator { 
        PermutationGenerator {
            size : n,
            current_perm: (1..=n).collect(),
            counters: vec![0; n],
            index: 0,
            first: true
        }
    }
}

impl Iterator for PermutationGenerator {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.current_perm.clone());
        }

        while self.index < self.size {
            if self.counters[self.index] < self.index {

                if self.index % 2 == 0 {
                    self.current_perm.swap(self.index, 0);
                } else {
                    self.current_perm.swap(self.index, self.counters[self.index]);
                }

                self.counters[self.index] += 1;
                self.index = 0;
                return Some(self.current_perm.clone());
            } else {
                self.counters[self.index] = 0;
                self.index += 1;
            }
        }
        None

    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use itertools::Itertools;
    use crate::algo::PermutationGenerator;

    #[test]
    fn it_works() {
        let pg = PermutationGenerator::new(3).into_iter().collect::<Vec<_>>(); 
        let itert  = (1usize ..=3).into_iter().permutations(3).collect::<Vec<_>>(); 

        assert_eq!(pg.len(), itert.len());
        assert_eq!(HashSet::<Vec<usize>>::from_iter(pg),HashSet::from_iter(itert));
    }

}