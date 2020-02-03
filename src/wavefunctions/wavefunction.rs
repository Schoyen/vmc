use crate::particle::Particles;

pub trait Wavefunction {
    fn get_num_parameters(&self) -> usize;
    fn get_parameters(&self) -> &Vec<f64>;

    fn evaluate(&self, particles: &Particles) -> f64;
    fn compute_laplacian(&self, particles: &Particles) -> f64;
}
