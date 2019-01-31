pub trait Wavefunction {
    // Constructor for a Wavefunction
    fn new(
        num_particles: u32,
        num_dimensions: u8,
        num_parameters: u8,
        mass: f64,
    ) -> Self;
    fn compute_laplacian(&self) -> f64;
    fn get_mass(&self) -> f64;
}
