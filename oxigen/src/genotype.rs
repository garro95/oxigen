//! This module contains the definition of genotypes.

use rand::prelude::SmallRng;
use std::iter::FromIterator;
use std::slice::Iter;

/// This trait defines an individual of a population in the genetic algorithm.
/// It defines the fitness and mutation functions and the type of the
/// individual representation.
pub trait Genotype<T>: FromIterator<T> + IntoIterator<Item = T> + Clone + Send + Sync {
    /// The type that represents the problem size of the genotype. For example,
    /// in the N Queens problem the size of the `ProblemSize` is a numeric type
    /// (the number of queens).
    type ProblemInstance: Send + Sync;

    /// Returns an iterator over the genes of the individual.
    fn iter(&self) -> Iter<T>;

    /// Randomly initiailzes an individual.
    fn generate(instance: &Self::ProblemInstance) -> Self;

    /// Returns a fitness value for an individual.
    fn fitness(&self) -> f64;

    /// Defines the manner in which an individual is mutated when
    /// an elemennt of the individual is selected to mutate.
    fn mutate(&mut self, rgen: &mut SmallRng, index: usize);

    /// Defines if an individual is a valid solution to the problem.
    fn is_solution(&self, fitness: f64) -> bool;

    /// Fix the individual to satisfy the problem restrictions. The default
    /// implementation is to remain the individual unmodified always.
    fn fix(&mut self) {}
}
