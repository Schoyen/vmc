use super::Wavefunction;
use crate::particle::{Particle, Particles};

#[derive(Debug)]
pub struct Gaussian {
    particles: Particles,
    num_parameters: usize,
    mass: f64,
}

impl Wavefunction for Gaussian {
    fn new(particles: Particles, num_parameters: usize, mass: f64) -> Self {
        Gaussian {
            particles,
            num_parameters,
            mass,
        }
    }

    fn compute_laplacian(&self) -> f64 {
        println!("Hello!");
        2.5
    }

    fn get_mass(&self) -> f64 {
        self.mass
    }
}
