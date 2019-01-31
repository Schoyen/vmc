use crate::wavefunctions::wavefunction;

#[derive(Debug)]
pub struct Gaussian {
    num_particles: u32,
    num_dimensions: u32,
    num_parameters: u32,
}

impl wavefunction::Wavefunction for Gaussian {
    fn new(
        num_particles: u32,
        num_dimensions: u32,
        num_parameters: u32,
    ) -> Self {
        Gaussian {
            num_particles,
            num_dimensions,
            num_parameters,
        }
    }
}
