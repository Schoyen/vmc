use crate::particle::{Particle, Particles};

pub trait Wavefunction {
    // Constructor for a Wavefunction
    fn new(particles: Particles, num_parameters: usize, mass: f64) -> Self;
    fn compute_laplacian(&self) -> f64;
    fn get_mass(&self) -> f64;
}
