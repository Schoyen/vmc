use super::Wavefunction;

#[derive(Debug)]
pub struct Gaussian {
    num_particles: u32,
    num_dimensions: u8,
    num_parameters: u8,
    mass: f64,
}

impl Wavefunction for Gaussian {
    fn new(
        num_particles: u32,
        num_dimensions: u8,
        num_parameters: u8,
        mass: f64,
    ) -> Self {
        Gaussian {
            num_particles,
            num_dimensions,
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
