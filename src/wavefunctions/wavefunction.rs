pub trait Wavefunction {
    fn new(num_particles: u32, num_dimensions: u8, num_parameters: u8) -> Self; // Constructor for a Wavefunction
}
