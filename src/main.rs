use wavefunctions::wavefunction::Wavefunction;

mod particle;
mod wavefunctions;

fn main() {
    let gaussian = wavefunctions::gaussian::Gaussian::new(1, 2, 3);
    dbg!(gaussian);

    let position: Vec<f64> = vec![1.0, 2.0, 3.0];
    let num_dimensions: u8 = 3;

    let particle = particle::Particle::new(position, num_dimensions);
    dbg!(particle);
}
