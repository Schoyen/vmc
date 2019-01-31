use crate::wavefunctions::wavefunction;

#[derive(Debug)]
pub struct Gaussian {
    num_particles: u32,
    num_dimensions: u8,
    num_parameters: u8,
}

impl wavefunction::Wavefunction for Gaussian {
    fn new(num_particles: u32, num_dimensions: u8, num_parameters: u8) -> Self {
        Gaussian {
            num_particles,
            num_dimensions,
            num_parameters,
        }
    }
}
