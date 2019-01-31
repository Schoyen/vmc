pub trait Wavefunction {
    fn new(
        num_particles: u32,
        num_dimensions: u32,
        num_parameters: u32,
    ) -> Self; // Constructor for a Wavefunction
}
