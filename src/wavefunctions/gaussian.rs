use super::Wavefunction;
use crate::particle::{Particle, Particles};

#[derive(Debug)]
pub struct Gaussian {
    parameters: Vec<f64>,
}

impl Gaussian {
    pub fn new(alpha: f64) -> Self {
        Gaussian {
            parameters: vec![alpha],
        }
    }
}

impl Wavefunction for Gaussian {
    fn get_num_parameters(&self) -> usize {
        self.parameters.len()
    }

    fn get_parameters(&self) -> &Vec<f64> {
        &self.parameters
    }

    fn evaluate(&self, particles: &Particles) -> f64 {
        1.0
    }

    fn compute_laplacian(&self, particles: &Particles) -> f64 {
        2.5
    }
}
