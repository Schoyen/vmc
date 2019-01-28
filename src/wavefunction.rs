pub trait Wavefunction {
    fn new(
        num_particles: u32,
        num_dimensions: u32,
        num_parameters: u32,
    ) -> Self; // Constructor for a Wavefunction
}

#[derive(Debug)]
pub struct Gaussian {
    num_particles: u32,
    num_dimensions: u32,
    num_parameters: u32,
}

impl Wavefunction for Gaussian {
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
