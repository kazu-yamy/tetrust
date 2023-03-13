use crate::ai::eval;
use crate::game::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::ops::Index;
// use std::thread;

// Number of Gene set
const POPULATION: usize = 10;
//maximum number of generations
const GENERATION_MAX: usize = 10;
// finish remove n line
const LINE_COUNT_MAX: usize = 256;

// gene type
pub enum GenomeKind {
    Line,
    HeightMax,
    HeightDiff,
    DeadSpace,
}

// gene sequence
pub type GenoSeq = [u8; 4];
impl Index<GenomeKind> for GenoSeq {
    type Output = u8;
    fn index(&self, kind: GenomeKind) -> &Self::Output {
        &self[kind as usize]
    }
}

// individual
#[derive(Clone)]
struct Individual {
    geno: GenoSeq,
    score: usize,
}

impl Distribution<Individual> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> Individual {
        Individual {
            geno: rand::random::<GenoSeq>(),
            score: 0,
        }
    }
}

// Learing
pub fn learning() {
    let mut inds = rand::random::<[Individual; POPULATION]>();
    for gen in 1..=GENERATION_MAX {
        println!("{gen}世代目");
        for (i, ind) in inds.iter().enumerate() {
            let mut game = Game::new();
            // finish remove n line
            while game.line < LINE_COUNT_MAX {
                let elite = eval(&game, &ind.geno);
                game = elite;
                // fall elite block
                if landing(&mut game).is_err() {
                    break;
                }
            }
            // show gene score
            println!("{i}: {:?} => {}", ind.geno, game.score);
        }
        // generate next generation
        let crossover = crossover(&inds); // cross over
        let mutation = mutation(&inds); // mutation
        let selection = selection(&inds);
        for (ind, selection) in inds.iter_mut().zip(selection) {
            ind.geno = selection;
        }
    }
    // finish
    quit();
}

// cross over
fn crossover(inds: &[Individual]) -> Vec<GenoSeq> {
    let genos = inds.iter().map(|i| i.geno).collect::<Vec<_>>();
    let mut new_genos = vec![];
    let mut rng = rand::thread_rng();
    for i in (0..genos.len() - 1).step_by(2) {
        let mut geno1 = genos[i];
        let mut geno2 = genos[i + 1];
        let point1 = rng.gen_range(0..4);
        let point2 = rng.gen_range(point1..4);
        mem_swap_range(&mut geno1, &mut geno2, point1..=point2);
        new_genos.push(geno1);
        new_genos.push(geno2);
    }
    new_genos
}

// exchanging data in a specified range
fn mem_swap_range<T>(x: &mut [T], y: &mut [T], range: std::ops::RangeInclusive<usize>) {
    for i in range {
        std::mem::swap(&mut x[i], &mut y[i]);
    }
}

// mutation
fn mutation(inds: &[Individual]) -> Vec<GenoSeq> {
    let mut genos = inds.iter().map(|i| i.geno).collect::<Vec<_>>();
    let mut rng = rand::thread_rng();
    for geno in genos.iter_mut() {
        geno[rng.gen_range(0..4)] = rand::random();
    }
    genos
}

// select
fn selection(inds: &[Individual]) -> Vec<GenoSeq> {
    let mut new_inds = inds.to_vec();
    new_inds.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    new_inds.iter().map(|i| i.geno).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_swap_range() {
        let tests = [
            (0..=0, [[5, 2, 3, 4], [1, 6, 7, 8]]),
            (0..=1, [[5, 6, 3, 4], [1, 2, 7, 8]]),
            (1..=1, [[1, 6, 3, 4], [5, 2, 7, 8]]),
            (1..=2, [[1, 6, 7, 4], [5, 2, 3, 8]]),
            (1..=3, [[1, 6, 7, 8], [5, 2, 3, 4]]),
            (0..=3, [[5, 6, 7, 8], [1, 2, 3, 4]]),
        ];
        for (range, [geno1_expect, geno2_expect]) in tests {
            let mut geno1 = [1, 2, 3, 4];
            let mut geno2 = [5, 6, 7, 8];
            mem_swap_range(&mut geno1, &mut geno2, range);
            assert_eq!(geno1, geno1_expect);
            assert_eq!(geno2, geno2_expect);
        }
    }
}
