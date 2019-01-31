#[derive(Debug)]
pub struct Particle {
    position: Vec<f64>,
    num_dimensions: u8,
}

impl Particle {
    pub fn new(position: Vec<f64>, num_dimensions: u8) -> Self {
        assert_eq!(position.len() as u8, num_dimensions);

        Particle {
            position,
            num_dimensions,
        }
    }

    pub fn move_particle(&mut self, step: f64, dim: u8) {
        self.position[dim as usize] += step;
    }
}
