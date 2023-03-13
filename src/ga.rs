use crate::ai::eval;
use crate::game::*;
use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    Rng,
};
use std::ops::Index;
use std::thread;

// Number of Gene set
const POPULATION: usize = 20;
//maximum number of generations
const GENERATION_MAX: usize = 20;
// finish remove n line
const LINE_COUNT_MAX: usize = 256;

// crossover rate
const CROSSOVER_RATE: usize = 70;
const CROSSOVER_LEN: usize = (POPULATION as f64 * (CROSSOVER_RATE as f64 / 100.)) as usize;
// mutation rate
const MUTATION_RATE: usize = 10;
const MUTATION_LEN: usize = (POPULATION as f64 * (MUTATION_RATE as f64 / 100.)) as usize;
// selection rate
const SELECTION_RATE: usize = 20;
const SELECTION_LEN: usize = (POPULATION as f64 * (SELECTION_RATE as f64 / 100.)) as usize;

// assert check rate
#[allow(clippy::assertions_on_constants)]
const _: () = assert!(CROSSOVER_RATE + MUTATION_RATE + SELECTION_RATE == 100);
#[allow(clippy::assertions_on_constants)]
const _: () = assert!(CROSSOVER_LEN + MUTATION_LEN + SELECTION_LEN == POPULATION);

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
        thread::scope(|s| {
            for (i, ind) in inds.iter_mut().enumerate() {
                s.spawn(move || {
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
                    // save score
                    ind.score = game.score;
                    // show gene score
                    println!("{i}: {:?} => {}", ind.geno, game.score);
                });
            }
        });
        // generate next generation
        let next_genos = gen_next_generation(&inds);
        // generational account
        inds.iter_mut()
            .map(|i| &mut i.geno)
            .zip(next_genos)
            .for_each(|(now, next)| *now = next);
    }
    // finish
    quit();
}

// generate next generation
fn gen_next_generation(inds: &[Individual]) -> [GenoSeq; POPULATION] {
    let mut rng = rand::thread_rng();
    let mut genos = vec![];
    genos.extend_from_slice(&crossover(inds)); // crossover
    genos.extend_from_slice(&mutation(inds)); //mutation
    genos.extend_from_slice(&selection(inds));
    genos.shuffle(&mut rng);
    genos.try_into().unwrap()
}

// cross over
fn crossover(inds: &[Individual]) -> [GenoSeq; CROSSOVER_LEN] {
    let mut genos = inds.iter().map(|i| i.geno).collect::<Vec<_>>();
    let mut rng = rand::thread_rng();
    for i in (0..genos.len() - 1).step_by(2) {
        let mut geno1 = genos[i];
        let mut geno2 = genos[i + 1];
        let point1 = rng.gen_range(0..4);
        let point2 = rng.gen_range(point1..4);
        mem_swap_range(&mut geno1, &mut geno2, point1..=point2);
        genos[i] = geno1;
        genos[i + 1] = geno2;
    }
    genos.shuffle(&mut rng);
    genos[..CROSSOVER_LEN].try_into().unwrap()
}

// exchanging data in a specified range
fn mem_swap_range<T>(x: &mut [T], y: &mut [T], range: std::ops::RangeInclusive<usize>) {
    for i in range {
        std::mem::swap(&mut x[i], &mut y[i]);
    }
}

// mutation
fn mutation(inds: &[Individual]) -> [GenoSeq; MUTATION_LEN] {
    let mut genos = inds.iter().map(|i| i.geno).collect::<Vec<_>>();
    let mut rng = rand::thread_rng();
    genos.shuffle(&mut rng);
    for geno in genos.iter_mut().take(MUTATION_LEN) {
        geno[rng.gen_range(0..4)] = rand::random();
    }
    genos[..MUTATION_LEN].try_into().unwrap()
}

// select
fn selection(inds: &[Individual]) -> [GenoSeq; SELECTION_LEN] {
    let mut new_inds = inds.to_vec();
    new_inds.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    new_inds.iter().map(|i| i.geno).collect::<Vec<_>>()[..SELECTION_LEN]
        .try_into()
        .unwrap()
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
