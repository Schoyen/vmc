use crate::particle::{Particle, Particles};

pub trait Wavefunction {
    fn get_num_parameters(&self) -> usize;
    fn get_parameters(&self) -> &Vec<f64>;

    fn evaluate(&self, particles: &Particles) -> f64;
    fn compute_gradient_of_particle(&self, particle: &Particle) -> Vec<f64>;
    fn compute_laplacian(&self, particles: &Particles) -> f64;
}
