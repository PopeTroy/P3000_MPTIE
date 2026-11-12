use rayon::prelude::*;
use crate::geodesic_solver::SentinelWalker;

pub struct SentinelScheduler {
    pub walkers: Vec<SentinelWalker>,
}

impl SentinelScheduler {
    pub fn new(walkers: Vec<SentinelWalker>) -> Self {
        Self { walkers }
    }

    pub fn execute_parallel_traversal(&mut self, steps: usize, d_tau: f64) {
        self.walkers.par_iter_mut().for_each(|walker| {
            for _ in 0..steps {
                walker.step_leapfrog(d_tau);
            }
        });
    }
}
